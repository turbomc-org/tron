use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{GetAllPrefixRequest, GetAllPrefixResponse, PartialPrefix as CompiledPrefix};

impl BridgeService {
    pub async fn handle_get_all_prefixes(
        &self,
        _request: Request<GetAllPrefixRequest>,
    ) -> Result<Response<GetAllPrefixResponse>, Status> {
        info!("Get all prefix request received");

        let prefixes = self.state().get_prefixes().await.map_err(|err| {
            error!("Failed to get all prefixes: {}", err);
            Status::internal("Failed to get all prefixes")
        })?;
        let complied_prefixes: Vec<CompiledPrefix> =
            prefixes.iter().map(|prefix| prefix.compile()).collect();

        info!("Get all prefix request completed");

        Ok(Response::new(GetAllPrefixResponse {
            prefixes: complied_prefixes,
        }))
    }
}
