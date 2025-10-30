use crate::collections::Collections;
use crate::state::State;
use bridge::bridge_server::Bridge;
use futures::Stream;

use once_cell::sync::Lazy;
use snowflaked::sync::Generator;
use std::iter::Take;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio_retry::strategy::ExponentialBackoff;
use tonic::Status;

pub mod collections;
pub mod grpc;
pub mod logger;
pub mod models;
pub mod modules;
pub mod requests;
pub mod state;
pub mod utils;

pub type ServerSendMessageStream = Pin<
    Box<
        dyn Stream<Item = Result<crate::bridge::ServerSendMessageResponse, Status>>
            + Send
            + 'static,
    >,
>;

pub type ServerSendTitleStream = Pin<
    Box<dyn Stream<Item = Result<crate::bridge::ServerSendTitleResponse, Status>> + Send + 'static>,
>;

pub type MessageStream =
    Pin<Box<dyn Stream<Item = Result<crate::bridge::MessageResponse, Status>> + Send>>;

static RETRY_STRATEGY: Lazy<Take<ExponentialBackoff>> = Lazy::new(|| {
    ExponentialBackoff::from_millis(100)
        .max_delay(Duration::from_secs(1))
        .take(3)
});

static GENERATOR: Generator = Generator::new(0);

pub mod bridge {
    tonic::include_proto!("bridge");
}

#[derive(Debug)]
pub struct BridgeService {
    state: Arc<State>,
    collections: Collections,
}

impl BridgeService {
    pub async fn new(collections: Collections) -> Self {
        let state = Arc::new(
            State::init(&collections)
                .await
                .expect("failed to load cache"),
        );

        Self { state, collections }
    }

    pub async fn broadcast_message(&self, msg: bridge::ServerSendMessageResponse) {
        let mut dead_clients = Vec::new();

        for entry in self.state.send_message_clients.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            if tx.send(Ok(msg.clone())).await.is_err() {
                dead_clients.push(client_id);
            }
        }

        for id in dead_clients {
            self.state.message_clients.remove(&id);
        }
    }

    pub async fn send_message_to_client(
        &self,
        client_id: u64,
        msg: bridge::ServerSendMessageResponse,
    ) -> Result<(), Status> {
        if let Some(entry) = self.state.send_message_clients.get(&client_id) {
            let tx = entry.value();
            tx.send(Ok(msg))
                .await
                .map_err(|_| Status::unavailable("Client disconnected"))?;
            Ok(())
        } else {
            Err(Status::not_found("Client not found"))
        }
    }

    fn filter_message(input: &str) -> String {
        let bad_words = ["badword", "noob"];
        let mut filtered = input.to_string();

        for word in bad_words {
            if filtered.to_lowercase().contains(word) {
                filtered = filtered.replace(word, "****");
            }
        }

        filtered
    }
}
