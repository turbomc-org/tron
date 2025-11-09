pub mod delete;
pub mod get;
pub mod get_all;
pub mod list_all;
pub mod view;

use crate::BridgeService;
use crate::bridge::{BugRequest, BugResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_bug(
        &self,
        request: Request<BugRequest>,
    ) -> Result<Response<BugResponse>, Status> {
        todo!("Implement bug functionality")
    }
}
