use crate::BridgeService;
use crate::bridge::{SendTeamInviteRequest, SendTeamInviteResponse};
use crate::models::team::Team;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

impl BridgeService {
    pub async fn handle_send_team_invite(
        &self,
        request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;

        info!("Send invite request from player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;
        let team = Team::get_team(&player, &self.cache.teams).await?;

        if team.leader != player.id {
            return Err(Status::permission_denied(
                "You are not the leader of this team",
            ));
        }

        let mut target_player = self.cache.get_player_with_handling(&target).await?;

        if target_player.team.is_some() {
            return Err(Status::already_exists("Target player is already in a team"));
        }

        let now = Utc::now().timestamp() as u64;

        target_player
            .add_team_invite(
                team.id,
                now,
                &self.databases.players,
                &self.cache.active_players,
            )
            .await
            .map_err(|err| {
                warn!("Failed to send team invite to {}: {}", target, err);
                Status::internal("Failed to send team invite")
            })?;

        info!("Send invite request from player {} completed", username);

        Ok(Response::new(SendTeamInviteResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::bridge_server::Bridge;
    use crate::bridge::{
        AcceptTeamInviteRequest, CreateTeamRequest, PlayerJoinRequest, SendTeamInviteRequest,
    };
    use crate::logger::Logger;
    use mongodb::bson::doc;
    use tonic::Request;

    #[tokio::test]
    async fn test_send_team_invite() {
        Logger::init(true).await;
        let service = BridgeService::new().await;

        let team_leader_name = "team_leader_send_1".to_string();
        let invited_player_name = "invited_player_send_1".to_string();
        let team_name = "cool_team_accept".to_string();
        let edition = 1;

        let _ = service
            .databases
            .players
            .delete_many(
                doc! {"$or": [{"username": &team_leader_name}, {"username": &invited_player_name}]},
            )
            .await;
        let _ = service
            .databases
            .teams
            .delete_one(doc! {"name": &team_name})
            .await;

        // 1. Create the team leader
        let leader_join_req = Request::new(PlayerJoinRequest {
            username: team_leader_name.clone(),
            edition,
        });
        assert!(
            service.player_join(leader_join_req).await.is_ok(),
            "Team leader should join successfully."
        );

        // 2. Create the player who will be invited
        let invited_join_req = Request::new(PlayerJoinRequest {
            username: invited_player_name.clone(),
            edition,
        });
        assert!(
            service.player_join(invited_join_req).await.is_ok(),
            "Invited player should join successfully."
        );

        // 3. The leader creates a team
        let create_team_req = Request::new(CreateTeamRequest {
            username: team_leader_name.clone(),
            team: team_name.clone(),
            color: "blue".to_string(),
            open: false, // Invite only
        });
        assert!(
            service.create_team(create_team_req).await.is_ok(),
            "Team creation should succeed."
        );

        // Allow some time for team creation to reflect in the cache
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        // 4. The leader invites the player to the team
        let invite_req = Request::new(SendTeamInviteRequest {
            username: team_leader_name.clone(),
            target: invited_player_name.clone(),
        });
        // We assume `invite_to_team` exists and works correctly for this test setup.
        // If it's not available, this test setup would need to manually insert the invite.
        assert!(
            service.handle_send_team_invite(invite_req).await.is_ok(),
            "Inviting player to team should succeed."
        );

        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let leader = service
            .databases
            .players
            .find_one(doc! {"username": &team_leader_name})
            .await
            .unwrap()
            .unwrap();

        let invited_player = service
            .databases
            .players
            .find_one(doc! {"username": &invited_player_name})
            .await
            .unwrap()
            .unwrap();

        assert!(
            invited_player
                .incoming_team_requests
                .contains_key(&leader.team.unwrap())
        )
    }
}
