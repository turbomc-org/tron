use crate::BridgeService;
use crate::bridge::{CreateRedeemCodeRequest, CreateRedeemCodeResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_create_redeem_code(
        &self,
        request: Request<CreateRedeemCodeRequest>,
    ) -> Result<Response<CreateRedeemCodeResponse>, Status> {
        todo!("Implement create redeem code")
    }
}
