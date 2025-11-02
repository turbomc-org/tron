use crate::BridgeService;
use crate::bridge::{AcceptTeamInviteRequest, AcceptTeamInviteResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_accept_team_invite(
        &self,
        request: Request<AcceptTeamInviteRequest>,
    ) -> Result<Response<AcceptTeamInviteResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.team;

        debug!(
            "Accept team invite request from player {} received",
            username
        );

        debug!("Fetching the player from cache");
        let mut player = self.state.get_player_with_handling(&username).await?;
        debug!("Fetching the team id");
        let team_id = self.state.check_team_request(&player, &target).await?;
        let now = Utc::now().timestamp() as u64;

        debug!("Accepting the team request");
        player
            .accept_team_request(
                team_id,
                now,
                &self.collections.players,
                &self.collections.teams,
                &self.state,
            )
            .await
            .map_err(|err| {
                error!(
                    "Failed to accept team invite request from player {}: {}",
                    username, err
                );
                Status::internal("Failed to accept team invite request")
            })?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>✅ SQUAD LINK ESTABLISHED</bold></gradient>\n\
             <gray>You have successfully joined the <white><bold>{}</bold></white> squad.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/team info'><u><gradient:#B200FF:#6A00A3>View Squad Roster</gradient></u></click>",
            target
          ),
        ).await;

        let team = self
            .state
            .get_team(team_id)
            .await
            .map_err(|_| Status::not_found("Team not found"))?
            .ok_or_else(|| Status::not_found("Team not found"))?;

        let team_broadcast_message = format!(
            "<gradient:#C724B1:#7A00FF><bold>⚡ CONNECTION ESTABLISHED</bold></gradient>\n\
             <gray><white><bold>{}</bold></white> has linked with your squad.</gray>\n\
             <dark_gray>»</dark_gray> <gray>Type <white>/tc</white> to welcome them.</gray>",
            username
        );

        for member in team
            .members
            .iter()
            .filter(|(member_id, _)| *member_id != &player.id)
        {
            let member_username = self
                .state
                .get_player_username(&member.0)
                .ok_or_else(|| Status::not_found("Member not found"))?;

            self.send_message_to_player(&member_username, team_broadcast_message.clone())
                .await
        }

        debug!(
            "Accept team invite request from player {} completed",
            username
        );

        Ok(Response::new(AcceptTeamInviteResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::bridge_server::Bridge;
//     use crate::bridge::{
//         AcceptTeamInviteRequest, CreateTeamRequest, PlayerJoinRequest, SendTeamInviteRequest,
//     };
//     use crate::logger::Logger;
//     use mongodb::bson::doc;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_handle_accept_team_invite() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let team_leader_name = "team_leader_accept_1".to_string();
//         let invited_player_name = "invited_player_accept_1".to_string();
//         let team_name = "cool_team_accept".to_string();
//         let edition = 1;

//         let _ = service
//             .databases
//             .players
//             .delete_many(
//                 doc! {"$or": [{"username": &team_leader_name}, {"username": &invited_player_name}]},
//             )
//             .await;
//         let _ = service
//             .databases
//             .teams
//             .delete_one(doc! {"name": &team_name})
//             .await;

//         let leader_join_req = Request::new(PlayerJoinRequest {
//             username: team_leader_name.clone(),
//             edition,
//         });
//         assert!(
//             service.player_join(leader_join_req).await.is_ok(),
//             "Team leader should join successfully."
//         );

//         let invited_join_req = Request::new(PlayerJoinRequest {
//             username: invited_player_name.clone(),
//             edition,
//         });
//         assert!(
//             service.player_join(invited_join_req).await.is_ok(),
//             "Invited player should join successfully."
//         );

//         let create_team_req = Request::new(CreateTeamRequest {
//             username: team_leader_name.clone(),
//             team: team_name.clone(),
//             color: "blue".to_string(),
//             open: false, // Invite only
//         });
//         assert!(
//             service.create_team(create_team_req).await.is_ok(),
//             "Team creation should succeed."
//         );

//         let invite_req = Request::new(SendTeamInviteRequest {
//             username: team_leader_name.clone(),
//             target: invited_player_name.clone(),
//         });

//         assert!(
//             service.handle_send_team_invite(invite_req).await.is_ok(),
//             "Inviting player to team should succeed."
//         );

//         let accept_invite_req = Request::new(AcceptTeamInviteRequest {
//             username: invited_player_name.clone(),
//             team: team_name.clone(),
//         });
//         let accept_response = service
//             .accept_team_invite(accept_invite_req)
//             .await
//             .expect("Accepting team invite should not fail")
//             .into_inner();

//         assert!(
//             accept_response.success,
//             "RPC response should indicate success."
//         );

//         tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

//         let leader = service
//             .databases
//             .players
//             .find_one(doc! {"username": &team_leader_name})
//             .await
//             .unwrap()
//             .unwrap();

//         let invited_player = service
//             .databases
//             .players
//             .find_one(doc! {"username": &invited_player_name})
//             .await
//             .unwrap()
//             .unwrap();

//         assert!(invited_player.team == leader.team);

//         // --- Cleanup Phase ---
//         let _ = service
//             .databases
//             .players
//             .delete_one(doc! {"username": &team_leader_name})
//             .await;
//         let _ = service
//             .databases
//             .players
//             .delete_one(doc! {"username": &invited_player_name})
//             .await;
//         let _ = service
//             .databases
//             .teams
//             .delete_one(doc! {"name": &team_name})
//             .await;
//     }
// }
