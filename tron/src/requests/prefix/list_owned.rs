use crate::BridgeService;
use crate::bridge::{ListOwnedPrefixRequest, ListOwnedPrefixResponse};
use crate::models::prefix::Prefix;
use tonic::{Request, Response, Status};
use tracing::error;
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_list_owned_prefix(
        &self,
        request: Request<ListOwnedPrefixRequest>,
    ) -> Result<Response<ListOwnedPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "List owned prefixes request from player {} received",
            username
        );

        let player = self.state.get_player_with_handling(&username).await?;
        let mut owned_prefixes: Vec<Prefix> = Vec::new();

        for prefix in player.prefixes {
            let prefix = self.state.get_prefix(&prefix).await.map_err(|err| {
                error!("Failed to retrieve prefix: {}", err);
                Status::internal("Failed to retrieve prefix")
            })?;

            owned_prefixes.push(prefix);
        }

        if owned_prefixes.is_empty() {
            self.send_message_to_player(
                &username,
                format!(
                    "<gradient:#C724B1:#7A00FF><bold>ℹ️ NO ASSETS UNLOCKED</bold></gradient>\n\
                     <gray>Your collection is empty. Acquire identifiers from the network market.</gray>\n\
                     <dark_gray>»</dark_gray> <click:run_command:'/shop prefixes'><u><gradient:#B200FF:#6A00A3>Browse the Network Market</gradient></u></click>"
                ),
            ).await;
        } else {
            let prefix_list_str = owned_prefixes
                .iter()
                .map(|prefix_data| {
                    let selected_prefix = player.selected_prefix;

                    if Some(prefix_data.id) == selected_prefix {
                        format!(
                            "<dark_gray> - <color:{}>{}</color> <gradient:#B200FF:#6A00A3><bold>[EQUIPPED]</bold></gradient></dark_gray>",
                            prefix_data.color, prefix_data.text
                        )
                    } else {
                        format!(
                            "<dark_gray> - <color:{}>{}</color> <click:run_command:'/prefix set {}'><u><gradient:#C724B1:#7A00FF>Equip</gradient></u></click></dark_gray>",
                            prefix_data.color, prefix_data.text, prefix_data.id
                        )
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            self.send_message_to_player(
                &username,
                format!(
                    "<gradient:#C724B1:#7A00FF><bold>🎨 IDENTIFIER COLLECTION</bold></gradient>\n\
                     <gray>Displaying your <white>{}</white> unlocked network assets:</gray>\n\
                     {}\n\
                     <dark_gray>»</dark_gray> <click:run_command:'/shop prefixes'><u><gradient:#B200FF:#6A00A3>Acquire more assets</gradient></u></click>",
                    owned_prefixes.len(),
                    prefix_list_str
                ),
            ).await;
        }

        let response_prefixes = owned_prefixes
            .iter()
            .map(|p| p.text.clone())
            .collect::<Vec<String>>();

        info!(
            "List owned prefixes request from player {} completed",
            username
        );

        Ok(Response::new(ListOwnedPrefixResponse {
            prefixes: response_prefixes,
        }))
    }
}
