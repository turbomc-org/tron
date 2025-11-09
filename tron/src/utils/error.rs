use crate::BridgeService;
use crate::config::messages::GENERIC_STATUS_ERROR;
use crate::render;
use tonic::Status;

impl BridgeService {
    pub async fn status<T>(&self, username: &str, status: Status) -> Result<T, Status> {
        let code = status.code().to_string().replace("_", " ");
        let message = status.message().to_string();

        let _ = self
            .send_message(
                username,
                render!(GENERIC_STATUS_ERROR, code = code, message = message),
            )
            .await;

        Err(status)
    }
}
