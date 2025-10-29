use crate::BridgeService;
use crate::bridge::{GetAllPrefixRequest, GetAllPrefixResponse, Prefix as CompiledPrefix};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_get_all_prefixes(
        &self,
        _request: Request<GetAllPrefixRequest>,
    ) -> Result<Response<GetAllPrefixResponse>, Status> {
        let prefixes = self.state.get_prefixes().await.map_err(|err| {
            error!("Failed to get all prefixes: {}", err);
            Status::internal("Failed to get all prefixes")
        })?;
        let complied_prefixes: Vec<CompiledPrefix> =
            prefixes.iter().map(|prefix| prefix.compile()).collect();

        Ok(Response::new(GetAllPrefixResponse {
            prefixes: complied_prefixes,
        }))
    }
}
