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
