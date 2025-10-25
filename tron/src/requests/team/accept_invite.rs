use crate::BridgeService;
use crate::bridge::{AcceptTeamInviteRequest, AcceptTeamInviteResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_accept_team_invite(
        &self,
        request: Request<AcceptTeamInviteRequest>,
    ) -> Result<Response<AcceptTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.team;

        info!(
            "Accept team invite request from player {} received",
            username
        );

        let mut player = self.cache.get_player_with_handling(&username).await?;
        let team_id = self.cache.check_team_request(&player, &target).await?;
        let now = Utc::now().timestamp() as u64;

        player
            .accept_team_request(
                team_id,
                now,
                &self.databases.players,
                &self.databases.teams,
                &self.cache.active_players,
                &self.cache.teams,
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to accept team invite request from player {}: {}",
                    username, err
                );
                Status::internal("Failed to accept team invite request")
            })?;

        info!(
            "Accept team invite request from player {} completed",
            username
        );

        Ok(Response::new(AcceptTeamInviteResponse { success: true }))
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
    async fn test_handle_accept_team_invite() {
        // --- Setup Phase ---
        Logger::init(true).await;
        let service = BridgeService::new().await;

        // Define unique names for test entities
        let team_leader_name = "team_leader_accept_1".to_string();
        let invited_player_name = "invited_player_accept_1".to_string();
        let team_name = "cool_team_accept".to_string();
        let edition = 1;

        // Clean up any potential leftover data from previous runs
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
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

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
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // --- Action Phase ---
        // 5. The invited player accepts the team invite
        let accept_invite_req = Request::new(AcceptTeamInviteRequest {
            username: invited_player_name.clone(),
            team: team_name.clone(),
        });
        let accept_response = service
            .accept_team_invite(accept_invite_req)
            .await
            .expect("Accepting team invite should not fail")
            .into_inner();

        // --- Verification Phase ---
        assert!(
            accept_response.success,
            "RPC response should indicate success."
        );

        // Allow time for asynchronous database and cache updates to complete
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        // 6. Verify database state
        let team_record = service
            .databases
            .teams
            .find_one(doc! {"name": &team_name})
            .await
            .unwrap()
            .expect("Team record must exist in the database.");

        let player_record = service
            .databases
            .players
            .find_one(doc! {"username": &invited_player_name})
            .await
            .unwrap()
            .expect("Player record must exist in the database.");

        // Assert that the player's team field is now set to the team's ID
        assert_eq!(
            player_record.team,
            Some(team_record.id),
            "Player's team field should be updated to the team's ID."
        );

        // Assert that the team's members list now includes the new player
        assert!(
            team_record.members.contains_key(&player_record.id),
            "Team's members map should contain the new player's ID."
        );

        // Assert that the player's incoming team request has been removed
        assert!(
            !player_record
                .incoming_team_requests
                .contains_key(&team_record.id),
            "Player's incoming team request should be removed."
        );

        // 7. Verify cache state
        let cached_player = service
            .cache
            .get_player(&invited_player_name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            cached_player.team,
            Some(team_record.id),
            "Cached player's team field should be updated."
        );

        let cached_team = service.cache.get_team_by_name(&team_name).await.unwrap();
        assert!(
            cached_team.members.contains_key(&player_record.id),
            "Cached team's members should include the new player."
        );

        // --- Cleanup Phase ---
        let _ = service
            .databases
            .players
            .delete_one(doc! {"username": &team_leader_name})
            .await;
        let _ = service
            .databases
            .players
            .delete_one(doc! {"username": &invited_player_name})
            .await;
        let _ = service
            .databases
            .teams
            .delete_one(doc! {"name": &team_name})
            .await;
    }
}
