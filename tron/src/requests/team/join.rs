use crate::BridgeService;
use crate::bridge::{JoinTeamRequest, JoinTeamResponse};
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_join_team(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team;

        info!("Join team request for player {} received", username);

        let mut player = self.cache.get_player_with_handling(&username).await?;

        if player.team.is_some() {
            error!("Player {} is already in a team", username);

            return Err(Status::failed_precondition(
                "You is already in a team. You must leave your current team before joining another.",
            ));
        }

        let mut team = self.cache.get_team_by_name(&team_name).await?;

        if !team.open {
            error!("Player {} tried to join a closed team", username);

            return Err(Status::failed_precondition(
                "Team is not open for everyone to join ask leader to send you an invitation",
            ));
        }

        let now = Utc::now().timestamp() as u64;

        team.add_member(
            &mut player,
            now,
            &self.collections.players,
            &self.collections.teams,
            &self.cache.active_players,
            &self.cache.teams,
        )
        .await
        .map_err(|err| {
            error!("Failed to join team: {}", err);
            Status::internal("Failed to join team")
        })?;

        info!("Join team request for player {} completed", username);

        Ok(Response::new(JoinTeamResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::bridge_server::Bridge;
//     use crate::bridge::{CreateTeamRequest, JoinTeamRequest, PlayerJoinRequest};
//     use crate::logger::Logger;
//     use mongodb::bson::doc;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_handle_join_team() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let team_leader_name = "team_leader_join_1".to_string();
//         let invited_player_name = "invited_player_join_1".to_string();
//         let team_name = "cool_team_join".to_string();
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
//             open: true,
//         });
//         assert!(
//             service.create_team(create_team_req).await.is_ok(),
//             "Team creation should succeed."
//         );

//         let join_req = Request::new(JoinTeamRequest {
//             username: invited_player_name.clone(),
//             team: team_name.clone(),
//         });

//         assert!(
//             service.handle_join_team(join_req).await.is_ok(),
//             "Joining player into open team should succeed."
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
