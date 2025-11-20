use crate::BridgeService;
use tonic::{Request, Response, Status};
use tron_protos::{GetAllRedeemCodesRequest, GetAllRedeemCodesResponse};

impl BridgeService {
    pub async fn handle_get_all_redeem_codes(
        &self,
        request: Request<GetAllRedeemCodesRequest>,
    ) -> Result<Response<GetAllRedeemCodesResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.player(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can get all redeem codes."),
                )
                .await;
        }

        let codes: Vec<String> = self
            .state()
            .indexes
            .redeem
            .iter()
            .map(|r| r.key().clone())
            .collect();

        Ok(Response::new(GetAllRedeemCodesResponse { codes }))
    }
}
