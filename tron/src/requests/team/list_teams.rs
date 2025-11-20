use crate::config::messages::TEAM_LIST;
use crate::models::team::Team;
use crate::render;
use crate::{BridgeService, config::messages::NO_TEAMS_FOUND};
use tonic::{Request, Response, Status};
use tron_protos::{ListTeamsRequest, ListTeamsResponse};

impl BridgeService {
    pub async fn handle_list_teams(
        &self,
        request: Request<ListTeamsRequest>,
    ) -> Result<Response<ListTeamsResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;

        let _ = self.player(&username).await;

        let teams: Vec<Team> = self
            .state()
            .teams
            .iter()
            .map(|t| t.value().clone())
            .collect();

        if teams.is_empty() {
            self.send_message(&username, render!(NO_TEAMS_FOUND, username = &username))
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
                     <gray>by</gray> <aqua>{}</aqua>",
                    rank, color, name, leader
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(TEAM_LIST, list = &list_str))
                .await;
        }

        Ok(Response::new(ListTeamsResponse { success: true }))
    }
}
