use crate::collections::Collections;
use crate::state::State;
use bridge::bridge_server::Bridge;
use futures::Stream;

use crate::utils::templates::render_template;
use once_cell::sync::Lazy;
use snowflaked::sync::Generator;
use std::iter::Take;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio_retry::strategy::ExponentialBackoff;
use tonic::Status;

pub mod collections;
pub mod config;
pub mod grpc;
pub mod logger;
pub mod models;
pub mod modules;
pub mod requests;
pub mod state;
pub mod utils;

pub type ProxyConnectionStream =
    Pin<Box<dyn Stream<Item = Result<crate::bridge::ProxyConnectionResponse, Status>> + Send>>;

static RETRY_STRATEGY: Lazy<Take<ExponentialBackoff>> = Lazy::new(|| {
    ExponentialBackoff::from_millis(100)
        .max_delay(Duration::from_secs(1))
        .take(3)
});

static GENERATOR: Generator = Generator::new(0);

#[derive(Debug)]
struct BridgeServiceInner {
    state: Arc<State>,
    collections: Collections,
}

pub mod bridge {
    tonic::include_proto!("bridge");
}

#[derive(Debug, Clone)]
pub struct BridgeService(Arc<BridgeServiceInner>);

impl BridgeService {
    pub async fn new(collections: Collections) -> Self {
        let state = Arc::new(
            State::init(&collections)
                .await
                .expect("failed to load cache"),
        );

        Self(Arc::new(BridgeServiceInner { state, collections }))
    }

    pub fn state(&self) -> &Arc<State> {
        &self.0.state
    }

    pub fn collections(&self) -> &Collections {
        &self.0.collections
    }
}
