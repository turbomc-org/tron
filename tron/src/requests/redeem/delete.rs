use crate::BridgeService;
use crate::bridge::{DeleteRedeemCodeRequest, DeleteRedeemCodeResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_delete_redeem_code(
        &self,
        request: Request<DeleteRedeemCodeRequest>,
    ) -> Result<Response<DeleteRedeemCodeResponse>, Status> {
        todo!("Implement delete redeem code")
    }
}
