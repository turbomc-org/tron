use crate::bridge::{ListOwnedPrefixRequest, ListOwnedPrefixResponse};
use crate::config::messages::{IDENTIFIER_COLLECTION, NO_ASSETS_UNLOCKED};
use crate::models::prefix::Prefix;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::error;
use tracing::info;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
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

        let player = self.state().get_player_with_handling(&username).await?;
        let mut owned_prefixes: Vec<Prefix> = Vec::new();

        for prefix in player.prefixes {
            let prefix = self.state().get_prefix(&prefix).await.map_err(|err| {
                error!("Failed to retrieve prefix: {}", err);
                Status::internal("Failed to retrieve prefix")
            })?;

            owned_prefixes.push(prefix);
        }

        if owned_prefixes.is_empty() {
            self.send_message(
                &username,
                render!(NO_ASSETS_UNLOCKED, username = &player.username),
            )
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();
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
                            "<dark_gray> - <color:{}>{}</color> <click:run_command:'/prefix equip {}'><u><gradient:#C724B1:#7A00FF>Equip</gradient></u></click></dark_gray>",
                            prefix_data.color, prefix_data.text, prefix_data.text
                        )
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            self.send_message(
                &username,
                render!(
                    IDENTIFIER_COLLECTION,
                    count = &owned_prefixes.len(),
                    list = &prefix_list_str
                ),
            )
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();
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
