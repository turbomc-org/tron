use crate::BridgeService;
use crate::bridge::{GetAllPrefixRequest, GetAllPrefixResponse, PartialPrefix as CompiledPrefix};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_get_all_prefixes(
        &self,
        request: Request<GetAllPrefixRequest>,
    ) -> Result<Response<GetAllPrefixResponse>, Status> {
        info!("Get all prefix request received");

        let prefixes = self.state.get_prefixes().await.map_err(|err| {
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
