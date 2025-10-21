use crate::BridgeService;
use crate::bridge::{BuyItemRequest, BuyItemResponse, SellItemRequest, SellItemResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_buy_item(
        &self,
        _request: Request<BuyItemRequest>,
    ) -> Result<Response<BuyItemResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_sell_item(
        &self,
        _request: Request<SellItemRequest>,
    ) -> Result<Response<SellItemResponse>, Status> {
        unimplemented!()
    }
}
