use crate::BridgeService;
use crate::bridge::{GetAllRedeemCodesRequest, GetAllRedeemCodesResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_redeem_codes(
        &self,
        request: Request<GetAllRedeemCodesRequest>,
    ) -> Result<Response<GetAllRedeemCodesResponse>, Status> {
        todo!("Implement delete redeem code")
    }
}
