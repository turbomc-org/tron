use crate::BridgeService;
use crate::utils::math::calculate_overall;
use tonic::{Request, Response, Status};
use tron_protos::{GetScoreboardRequest, GetScoreboardResponse};

impl BridgeService {
    pub async fn handle_get_scoreboard(
        &self,
        request: Request<GetScoreboardRequest>,
    ) -> Result<Response<GetScoreboardResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.player(&username).await?;

        let overall = match self.state().leaderboards.overall.get_rank(&player.id).await {
            Some(rank) => rank,
            None => {
                self.status(&username, Status::internal("Failed to fetch overall rank."))
                    .await?
            }
        };

        let kills = match self.state().leaderboards.kills.get_rank(&player.id).await {
            Some(rank) => rank,
            None => {
                self.status(&username, Status::internal("Failed to fetch kills rank."))
                    .await?
            }
        };

        let deaths = match self.state().leaderboards.deaths.get_rank(&player.id).await {
            Some(rank) => rank,
            None => {
                self.status(&username, Status::internal("Failed to fetch deaths rank."))
                    .await?
            }
        };

        let kd = match self.state().leaderboards.kd.get_rank(&player.id).await {
            Some(rank) => rank,
            None => {
                self.status(&username, Status::internal("Failed to fetch kd rank."))
                    .await?
            }
        };

        Ok(Response::new(GetScoreboardResponse {
            rank: player.compile_rank(),
            kills: player.kills,
            deaths: player.deaths,
            coins: player.coins,
            overall: calculate_overall(player.kills, player.deaths, player.coins) as u64,
            rank_in_deaths_leaderboard: deaths as u64,
            rank_in_kills_leaderboard: kills as u64,
            rank_in_overall_leaderboard: overall as u64,
            rank_in_kd_leaderboard: kd as u64,
        }))
    }
}
