use crate::BridgeService;
use crate::bridge::{ListKdaLeaderboardRequest, ListKdaLeaderboardResponse};
use crate::config::messages::{KD_LEADERBOARD, KD_LEADERBOARD_EMPTY};
use crate::render;
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_list_kda_leaderboard(
        &self,
        request: Request<ListKdaLeaderboardRequest>,
    ) -> Result<Response<ListKdaLeaderboardResponse>, Status> {
        let inner = request.into_inner();
        let username = inner.username;

        let player = self.state().get_player_with_handling(&username).await?;

        let leaderboard = self.state().leaderboards.kd.get(10).await;

        if leaderboard.is_empty() {
            self.send_message(
                &username,
                render!(KD_LEADERBOARD_EMPTY, username = username),
            )
            .await
            .map_err(|err| {
                error!("Failed to send leaderboard empty message: {}", err);
            })
            .ok();

            return Ok(Response::new(ListKdaLeaderboardResponse { success: true }));
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
                "<gray>#{}</gray> <aqua>{}</aqua> <dark_gray>-</dark_gray> <yellow><bold>{}</bold></yellow> kd",
                index + 1,
                name,
                coins
            ));
        }

        let list_msg = entries.join("\n");

        let rank = self.state().leaderboards.kd.get_rank(&player.id).await;
        let rank_display = rank.map(|r| r.to_string()).unwrap_or("Unranked".into());

        self.send_message(
            &username,
            render!(KD_LEADERBOARD, list = &list_msg, rank = &rank_display),
        )
        .await
        .map_err(|err| {
            error!("Failed to send kd leaderboard message: {}", err);
        })
        .ok();

        Ok(Response::new(ListKdaLeaderboardResponse { success: true }))
    }
}
