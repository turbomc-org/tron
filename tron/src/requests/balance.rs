use crate::BridgeService;
use crate::bridge::{
    GetBalanceRequest, GetBalanceResponse, TransferBalanceRequest, TransferBalanceResponse,
};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_transfer_balance(
        &self,
        request: Request<TransferBalanceRequest>,
    ) -> Result<Response<TransferBalanceResponse>, Status> {
        unimplemented!()
    }
}
