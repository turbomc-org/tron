use crate::BridgeService;
use crate::bridge::{IncreaseCoinsRequest, IncreaseCoinsResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_increase_coins(
        &self,
        request: Request<IncreaseCoinsRequest>,
    ) -> Result<Response<IncreaseCoinsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let target = inner_request.target;
        let amount = inner_request.amount;

        let _ = self.state.get_player_with_handling(&username).await?;
        let mut target_player = self.state.get_player_with_handling(&target).await?;

        target_player
            .inc_coins(amount, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!(
                    "Failed to increase {} coins to {}'s balance: {}",
                    amount, target, err
                );

                Status::internal(format!(
                    "Failed to increase {} coins to {}'s balance",
                    amount, target,
                ))
            })?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>MASTER CONTROL: CREDITS GRANTED</bold></gradient>\n\
             <gray>You have granted <white><bold>{}</bold></white> Data-Credits to the user <white><bold>{}</bold></white>.</gray>",
            amount, target
          ),
        ).await;

        self.send_message_to_player(
          &target,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>⚡ ADMINISTRATIVE GRANT</bold></gradient>\n\
             <gray>An administrator has updated your account with <white><bold>{}</bold></white> Data-Credits.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/balance'><u><gradient:#B200FF:#6A00A3>Check your new balance</gradient></u></click>",
            amount
          ),
        ).await;

        Ok(Response::new(IncreaseCoinsResponse { success: true }))
    }
}
