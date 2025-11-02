use crate::BridgeService;
use crate::bridge::{BuyPrefixRequest, BuyPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_buy_prefix(
        &self,
        request: Request<BuyPrefixRequest>,
    ) -> Result<Response<BuyPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_name = inner_request.prefix;

        info!("Buy prefix request from player {} received", username);

        let mut player = self.state.get_player_with_handling(&username).await?;
        let prefix = self.state.get_prefix_with_handling(&prefix_name).await?;

        if player.prefixes.contains(&prefix.id) {
            self.send_message_to_player(
              &username,
              format!(
                "<gradient:#FF4D4D:#FF0000><bold>❌ DUPLICATE ASSET</bold></gradient>\n\
                 <gray>You have already unlocked this network identifier.</gray>\n\
                 <dark_gray>»</dark_gray> <click:run_command:'/prefixes'><u><gradient:#C724B1:#7A00FF>View your collection</gradient></u></click>"
              ),
            ).await;

            error!("Player already owns this prefix");
            return Err(Status::already_exists("You already owns this prefix"));
        }

        if player.coins < prefix.price {
            self.send_message_to_player(
              &username,
              format!(
                "<gradient:#FF4D4D:#FF0000><bold>❌ INSUFFICIENT CREDITS</bold></gradient>\n\
                 <gray>Your balance is too low to acquire this asset. You need <white>{}</white> more credits.</gray>\n\
                 <dark_gray>»</dark_gray> <click:run_command:'/balance'><u><gradient:#C724B1:#7A00FF>Check your balance</gradient></u></click>",
                prefix.price - player.coins
              ),
            ).await;

            error!("Player does not have enough coins");
            return Err(Status::failed_precondition("You do not have enough coins"));
        }

        prefix
            .buy(&mut player, &self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to buy prefix: {}", err);
                Status::internal("Failed to buy prefix")
            })?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>✅ ASSET ACQUIRED</bold></gradient>\n\
             <gray>You purchased the <color:{}>{}</color> <gray>prefix for <white>{}</white> credits.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/prefix set {}'><u><gradient:#B200FF:#6A00A3>Click to equip</gradient></u></click>",
            prefix.color,
            prefix.text,
            prefix.price,
            prefix_name
          ),
        ).await;

        info!("Buy prefix request from player {} completed", username);

        Ok(Response::new(BuyPrefixResponse { success: true }))
    }
}
