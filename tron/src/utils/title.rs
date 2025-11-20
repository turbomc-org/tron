use crate::BridgeService;
use tron_protos::TitlePlayer;

impl BridgeService {
    pub async fn send_title(&self, username: &str, title: String, subtitle: String) {
        self.title_player(TitlePlayer {
            username: username.to_string(),
            title: title,
            subtitle: subtitle,
        })
        .await;
    }
}
