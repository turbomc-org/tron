use crate::BridgeService;
use crate::config::messages::JOINED_CHANNEL;
use crate::render;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{TeamChatRequest, TeamChatResponse};

impl BridgeService {
    pub async fn handle_team_chat(
        &self,
        request: Request<TeamChatRequest>,
    ) -> Result<Response<TeamChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Team chat request from player {} completed", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.team.is_none() {
            return self
                .status(
                    &username,
                    Status::not_found("You are not currently in any team."),
                )
                .await;
        }

        let team = self
            .state()
            .get_team_with_handling(player.team.unwrap())
            .await?;

        self.state().messaging.join_team(player.id, team.id);

        let mut active_members = Vec::new();

        for member in team.members {
            if let Some(username) = self.state().get_player_username(&member.0) {
                if self.state().active_players.contains_key(&username) {
                    active_members.push(username.clone());
                }
            }
        }

        for player in active_members {
            if let Err(e) = self
                .send_message(&player, render!(JOINED_CHANNEL, username = &player))
                .await
            {
                error!("Failed to send message to player {}: {}", player, e);
                return Err(Status::internal("Failed to send message"));
            }
        }

        info!("Team chat request from player {} completed", username);

        Ok(Response::new(TeamChatResponse { success: true }))
    }
}
