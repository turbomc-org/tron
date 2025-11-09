pub mod create;
pub mod delete;
pub mod get_all;
pub mod list_all;

use crate::BridgeService;
use crate::bridge::{RedeemCodeRequest, RedeemCodeResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_redeem_code(
        &self,
        request: Request<RedeemCodeRequest>,
    ) -> Result<Response<RedeemCodeResponse>, Status> {
        todo!("Implement list all redeem codes")
    }
}
