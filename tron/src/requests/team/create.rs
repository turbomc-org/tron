use crate::BridgeService;
use crate::bridge::{CreateTeamRequest, CreateTeamResponse};
use crate::models::team::Team;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_create_team(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team;
        let color = inner_request.color;
        let open = inner_request.open;

        info!("Create team request from player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;

        #[derive(Serialize, Deserialize)]
        struct PartialResponse {
            #[serde(rename = "_id")]
            id: u64,
        }

        let partial_teams: Collection<PartialResponse> = self.databases.players.clone_with_type();
        let projection = doc! { "_id": 1 };
        let find_options = FindOneOptions::builder().projection(projection).build();

        match partial_teams
            .find_one(doc! {"name": team_name.clone()})
            .with_options(find_options)
            .await
        {
            Ok(Some(_)) => {
                error!("Team already exists with name {}", team_name);

                Err(Status::already_exists(format!(
                    "Team already exists with name {}",
                    team_name,
                )))
            }
            Ok(None) => {
                let team = Team::new(team_name.clone(), player.id, open, color);

                team.insert(&self.databases.teams, &self.cache.teams)
                    .await
                    .map_err(|err| {
                        error!("Failed to create team: {}", err);

                        Status::internal(format!("Failed to create team: {}", err))
                    })?;

                Ok(Response::new(CreateTeamResponse { success: true }))
            }
            Err(e) => {
                error!("Failed to check team's existence: {}", e);

                Err(Status::internal(format!(
                    "Failed to check team existence: {}",
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::bridge::{CreateTeamRequest, PlayerJoinRequest};
    use mongodb::bson::doc;
    use tonic::Request;
    use tracing::info;

    #[tokio::test]
    async fn test_handle_create_team() {
        // === Step 1: Initialize ===
        let service = BridgeService::new().await;
        tracing_subscriber::fmt::try_init().ok();

        // Clean up possible leftover test data from previous runs
        let _ = service
            .databases
            .teams
            .delete_one(doc! {"name": "test_team_create"})
            .await;

        let _ = service
            .databases
            .players
            .delete_one(doc! {"username": "test_creator"})
            .await;

        // === Step 2: Create a player ===
        let join_req = Request::new(PlayerJoinRequest {
            username: "test_creator".to_string(),
            edition: 1,
        });

        let join_resp = service.player_join(join_req).await.unwrap().into_inner();
        assert!(
            join_resp.success,
            "Player should be able to join before team creation"
        );

        // === Step 3: Create a team ===
        let create_req = Request::new(CreateTeamRequest {
            username: "test_creator".to_string(),
            team: "test_team_create".to_string(),
            color: "red".to_string(),
            open: true,
        });

        let response = service
            .handle_create_team(create_req)
            .await
            .expect("Team creation should not fail")
            .into_inner();

        assert!(response.success, "Team creation must return success = true");

        // === Step 4: Wait for async insert to complete ===
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // === Step 5: Verify DB entry ===
        let team_doc = service
            .databases
            .teams
            .find_one(doc! {"name": "test_team_create"})
            .await
            .unwrap();

        assert!(team_doc.is_some(), "Team should be inserted into DB");

        let team = team_doc.unwrap();
        assert_eq!(team.name, "test_team_create");
        assert_eq!(team.color, "red");
        assert!(
            team.members.contains_key(&team.leader),
            "Owner should be in team members"
        );

        // === Step 6: Verify cache ===
        let cached_team = service
            .cache
            .get_team_by_name("test_team_create")
            .await
            .expect("Team should exist in cache");

        assert_eq!(
            cached_team.name, "test_team_create",
            "Cached team should match the created one"
        );

        info!("✅ Team successfully created and verified in DB + cache");

        // === Step 7: Cleanup ===
        service
            .databases
            .teams
            .delete_one(doc! {"name": "test_team_create"})
            .await
            .unwrap();

        service
            .databases
            .players
            .delete_one(doc! {"username": "test_creator"})
            .await
            .unwrap();
    }
}
