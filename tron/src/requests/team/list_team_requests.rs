use crate::config::messages::TEAM_REQUESTS_LIST;
use crate::models::team::Team;
use crate::render;
use crate::{BridgeService, config::messages::NO_TEAM_REQUESTS_FOUND};
use tonic::{Request, Response, Status};
use tron_protos::{ListTeamRequestsRequest, ListTeamRequestsResponse};

impl BridgeService {
    pub async fn handle_list_team_requests(
        &self,
        request: Request<ListTeamRequestsRequest>,
    ) -> Result<Response<ListTeamRequestsResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;

        let player = self.player(&username).await?;

        let mut teams: Vec<Team> = Vec::new();
        let request_teams_id: Vec<u64> = player
            .incoming_team_requests
            .iter()
            .map(|r| r.0.clone())
            .collect();

        for request_team_id in request_teams_id {
            let request_team = self.team_by_id(&username, &request_team_id).await?;
            teams.push(request_team);
        }

        if teams.is_empty() {
            self.send_message(
                &username,
                render!(NO_TEAM_REQUESTS_FOUND, username = &username),
            )
            .await;
        } else {
            let mut entries = Vec::new();

            for team in teams {
                let rank = self
                    .state()
                    .leaderboards
                    .teams
                    .get_rank(&team.id)
                    .await
                    .unwrap_or(99);
                let name = team.name;
                let color = team.color;
                let leader = self
                    .state()
                    .get_player_username(&team.leader)
                    .unwrap_or("Unknown".to_string());

                entries.push(format!(
                    "<dark_gray>»</dark_gray> <yellow><bold>#{}</bold></yellow> \
                     <gray>-</gray> <color:{}>{}</color> \
                     <gray>by</gray> <aqua>{}</aqua>\n\
                     [<red><click:run_command:'/team accept {}'>Accept</click></red>]\n\
                     [<red><click:run_command:'/team decline {}'>Decline</click></red>]\n\
                     ",
                    rank, color, name, leader, name, name
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(TEAM_REQUESTS_LIST, list = &list_str))
                .await;
        }

        Ok(Response::new(ListTeamRequestsResponse { success: true }))
    }
}
