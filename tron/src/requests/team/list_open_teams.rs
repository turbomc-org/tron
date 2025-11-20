use crate::BridgeService;
use crate::config::messages::{NO_OPEN_TEAMS_FOUND, OPEN_TEAM_LIST};
use crate::models::team::Team;
use crate::render;
use tonic::{Request, Response, Status};
use tron_protos::{ListOpenTeamsRequest, ListOpenTeamsResponse};

impl BridgeService {
    pub async fn handle_list_open_teams(
        &self,
        request: Request<ListOpenTeamsRequest>,
    ) -> Result<Response<ListOpenTeamsResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;

        let _ = self.player(&username).await;

        let mut teams: Vec<Team> = Vec::new();
        let open_teams_id = self.state().indexes.open_team.clone();

        for open_team_id in open_teams_id {
            let open_team = self.team_by_id(&username, &open_team_id).await?;
            teams.push(open_team);
        }

        if teams.is_empty() {
            self.send_message(
                &username,
                render!(NO_OPEN_TEAMS_FOUND, username = &username),
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
                     [<red><click:run_command:'/team join {}'>Join</click></red>]",
                    rank, color, name, leader, name
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(OPEN_TEAM_LIST, list = &list_str))
                .await;
        }

        Ok(Response::new(ListOpenTeamsResponse { success: true }))
    }
}
