use crate::BridgeService;
use crate::config::messages::{OVERALL_LEADERBOARD, OVERALL_LEADERBOARD_EMPTY};
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{ListOverallLeaderboardRequest, ListOverallLeaderboardResponse};

impl BridgeService {
    pub async fn handle_list_overall_leaderboard(
        &self,
        request: Request<ListOverallLeaderboardRequest>,
    ) -> Result<Response<ListOverallLeaderboardResponse>, Status> {
        let inner = request.into_inner();
        let username = inner.username;

        info!(
            "List Overall Leaderboard request from player {} received",
            username
        );

        let player = self.player(&username).await?;

        let leaderboard = self.state().leaderboards.overall.get(10).await;

        if leaderboard.is_empty() {
            self.send_message(
                &username,
                render!(OVERALL_LEADERBOARD_EMPTY, username = username),
            )
            .await;

            return Ok(Response::new(ListOverallLeaderboardResponse {
                success: true,
            }));
        }

        let mut entries = Vec::new();

        for (index, entry) in leaderboard.iter().enumerate() {
            let player_id = entry.0;
            let coins = entry.1;

            let name = match self.state().get_player_username(&player_id) {
                Some(n) => n,
                None => format!("Unknown ({})", player_id),
            };

            entries.push(format!(
                "<gray>#{}</gray> <aqua>{}</aqua> <dark_gray>-</dark_gray> <yellow><bold>{}</bold></yellow> points",
                index + 1,
                name,
                coins
            ));
        }

        let list_msg = entries.join("\n");

        let rank = self.state().leaderboards.overall.get_rank(&player.id).await;
        let rank_display = rank.map(|r| r.to_string()).unwrap_or("Unranked".into());

        self.send_message(
            &username,
            render!(OVERALL_LEADERBOARD, list = &list_msg, rank = &rank_display),
        )
        .await;

        info!(
            "List Overall Leaderboard request from player {} completed",
            username
        );

        Ok(Response::new(ListOverallLeaderboardResponse {
            success: true,
        }))
    }
}
