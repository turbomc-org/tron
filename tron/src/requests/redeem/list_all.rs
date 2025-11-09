use crate::BridgeService;
use crate::bridge::{ListAllRedeemCodesRequest, ListAllRedeemCodesResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_list_all_redeem_codes(
        &self,
        request: Request<ListAllRedeemCodesRequest>,
    ) -> Result<Response<ListAllRedeemCodesResponse>, Status> {
        todo!("Implement list all redeem codes")
    }
}
