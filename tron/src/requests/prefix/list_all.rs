use crate::BridgeService;
use crate::bridge::{ListAllPrefixRequest, ListAllPrefixResponse, PartialPrefix as CompiledPrefix};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_list_all_prefix(
        &self,
        request: Request<ListAllPrefixRequest>,
    ) -> Result<Response<ListAllPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("List all prefix request received");

        let player = self.state.get_player_with_handling(&username).await?;

        let prefixes = self.state.get_prefixes().await.map_err(|err| {
            error!("Failed to get all prefixes: {}", err);
            Status::internal("Failed to get all prefixes")
        })?;
        let complied_prefixes: Vec<CompiledPrefix> =
            prefixes.iter().map(|prefix| prefix.compile()).collect();

        if prefixes.is_empty() {
            self.send_message_to_player(
                &username,
                format!(
                    "<gradient:#C724B1:#7A00FF><bold>ℹ️ MARKET DATABASE EMPTY</bold></gradient>\n\
                     <gray>There are currently no network identifiers available for acquisition.</gray>\n\
                     <dark_gray>»</dark_gray> <gray>Please check back later.</gray>"
                ),
            ).await;
        } else {
            let player_owned_prefixes: std::collections::HashSet<String> = player
                .prefixes
                .iter()
                .map(|id| {
                    self.state
                        .get_prefix_text(id)
                        .ok_or("undefined".to_string())
                })
                .filter_map(|res| res.ok())
                .collect();

            let prefix_list_str = prefixes
                .iter()
                .map(|prefix| {
                    if player_owned_prefixes.contains(&prefix.text) {
                        format!(
                            "<dark_gray> - {} <gradient:#B200FF:#6A00A3><bold>[UNLOCKED]</bold></gradient></dark_gray>",
                            prefix.text
                        )
                    } else {
                        format!(
                            "<dark_gray> - <color:{}>{}</color> <dark_gray>| <white>{} ¤</white> <click:run_command:'/prefix buy {}'><u><gradient:#C724B1:#7A00FF>Acquire</gradient></u></click></dark_gray>",
                            prefix.color, prefix.text, prefix.price, prefix.text
                        )
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            self.send_message_to_player(
                &username,
                format!(
                    "<gradient:#C724B1:#7A00FF><bold>🌐 NETWORK MARKET: IDENTIFIERS</bold></gradient>\n\
                     <gray>Displaying all available network assets:</gray>\n\
                     {}\n\
                     <dark_gray>»</dark_gray> <click:run_command:'/prefixes'><u><gradient:#B200FF:#6A00A3>View your owned assets</gradient></u></click>",
                    prefix_list_str
                ),
            ).await;
        }

        info!("List all prefix request completed");

        Ok(Response::new(ListAllPrefixResponse {
            prefixes: complied_prefixes,
        }))
    }
}
