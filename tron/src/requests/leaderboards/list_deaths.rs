use crate::BridgeService;
use crate::config::messages::{DEATHS_LEADERBOARD, DEATHS_LEADERBOARD_EMPTY};
use crate::render;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{ListDeathsLeaderboardRequest, ListDeathsLeaderboardResponse};

impl BridgeService {
    pub async fn handle_list_deaths_leaderboard(
        &self,
        request: Request<ListDeathsLeaderboardRequest>,
    ) -> Result<Response<ListDeathsLeaderboardResponse>, Status> {
        let inner = request.into_inner();
        let username = inner.username;

        info!(
            "List Deaths Leaderboard request from player {} received",
            username
        );

        let player = self.player(&username).await?;

        let leaderboard = self.state().leaderboards.deaths.get(10).await;

        if leaderboard.is_empty() {
            self.send_message(
                &username,
                render!(DEATHS_LEADERBOARD_EMPTY, username = username),
            )
            .await;

            return Ok(Response::new(ListDeathsLeaderboardResponse {
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
                "<gray>#{}</gray> <aqua>{}</aqua> <dark_gray>-</dark_gray> <yellow><bold>{}</bold></yellow> deaths",
                index + 1,
                name,
                coins
            ));
        }

        let list_msg = entries.join("\n");

        let rank = self.state().leaderboards.coins.get_rank(&player.id).await;
        let rank_display = rank.map(|r| r.to_string()).unwrap_or("Unranked".into());

        self.send_message(
            &username,
            render!(DEATHS_LEADERBOARD, list = &list_msg, rank = &rank_display),
        )
        .await;

        info!(
            "List Deaths Leaderboard request from player {} completed",
            username
        );

        Ok(Response::new(ListDeathsLeaderboardResponse {
            success: true,
        }))
    }
}
