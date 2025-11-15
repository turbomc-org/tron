use crate::BridgeService;
use crate::bridge::{GetAllRedeemCodesRequest, GetAllRedeemCodesResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_all_redeem_codes(
        &self,
        request: Request<GetAllRedeemCodesRequest>,
    ) -> Result<Response<GetAllRedeemCodesResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state().get_player_with_handling(&username).await?;

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
