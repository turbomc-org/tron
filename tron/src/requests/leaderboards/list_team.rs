use crate::BridgeService;
use crate::config::messages::{TEAMS_LEADERBOARD, TEAMS_LEADERBOARD_EMPTY};
use crate::models::team::Team;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{ListTeamsLeaderboardRequest, ListTeamsLeaderboardResponse};

impl BridgeService {
    pub async fn handle_list_teams_leaderboard(
        &self,
        request: Request<ListTeamsLeaderboardRequest>,
    ) -> Result<Response<ListTeamsLeaderboardResponse>, Status> {
        let inner = request.into_inner();
        let username = inner.username;

        info!(
            "List Team Leaderboard request from player {} received",
            username
        );

        let player = self.player(&username).await?;

        if player.team.is_none() {
            return self
                .status(
                    &username,
                    Status::failed_precondition("You are not in any team."),
                )
                .await;
        }

        let leaderboard = self.state().leaderboards.teams.get(10).await;

        if leaderboard.is_empty() {
            self.send_message(
                &username,
                render!(TEAMS_LEADERBOARD_EMPTY, username = username),
            )
            .await;

            return Ok(Response::new(ListTeamsLeaderboardResponse {
                success: true,
            }));
        }

        let mut entries = Vec::new();

        for (index, entry) in leaderboard.iter().enumerate() {
            let team_id = entry.0;
            let overall = entry.1;

            let team: Team = match self.state().get_team(team_id) {
                Some(n) => n,
                None => continue,
            };

            entries.push(format!(
                "<gray>#{}</gray> <aqua>{}</aqua> <dark_gray>-</dark_gray> <yellow><bold>{}</bold></yellow> points",
                index + 1,
                team.name,
                overall
            ));
        }

        let list_msg = entries.join("\n");

        let rank = self.state().leaderboards.overall.get_rank(&player.id).await;
        let rank_display = rank.map(|r| r.to_string()).unwrap_or("Unranked".into());

        self.send_message(
            &username,
            render!(TEAMS_LEADERBOARD, list = &list_msg, rank = &rank_display),
        )
        .await;

        info!(
            "List Team Leaderboard request from player {} completed",
            username
        );

        Ok(Response::new(ListTeamsLeaderboardResponse {
            success: true,
        }))
    }
}
