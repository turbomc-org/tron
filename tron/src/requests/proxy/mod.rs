use crate::bridge::ProxyConnectionRequest;
use crate::{BridgeService, ProxyConnectionStream};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::error;

pub mod ban_player;
pub mod emit_message;
pub mod emit_whisper;
pub mod heartbeat;
pub mod kick_player;
pub mod message_player;
pub mod title_player;
pub mod transfer_player;

pub struct ProxyConnection {}

impl BridgeService {
    pub async fn handle_proxy_connection(
        &self,
        request: Request<Streaming<ProxyConnectionRequest>>,
    ) -> Result<Response<ProxyConnectionStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        let state = self.state().clone();

        if let Some(Ok(first_msg)) = in_stream.next().await {
            if let Some(payload) = first_msg.payload {
                match payload {
                    crate::bridge::proxy_connection_request::Payload::Handshake(handshake) => {
                        let proxy_id = handshake.proxy_id;
                        state.proxy_connections.insert(proxy_id, tx.clone());

                        let this = self.clone();

                        tokio::spawn(async move {
                            while let Some(result) = in_stream.next().await {
                                match result {
                                    Ok(v) => {
                                        if let Some(payload) = v.payload {
                                            match payload {
                                                crate::bridge::proxy_connection_request::Payload::Heartbeat(payload) => {
                                                    this.handle_proxy_heartbeat(payload).await;
                                                }
                                                crate::bridge::proxy_connection_request::Payload::EmitMessage(payload) => {
                                                    this.handle_proxy_emit_message(payload).await
                                                }
                                                crate::bridge::proxy_connection_request::Payload::EmitWhisper(payload) => {
                                                    this.handle_proxy_emit_whisper(payload).await
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        error!(
                                            "Failed to handle proxy connection request: {}",
                                            err
                                        );
                                        break;
                                    }
                                }
                            }
                            state.proxy_connections.remove(&proxy_id);
                        });
                    }
                    _ => {
                        error!("Invalid proxy connection request");
                    }
                }
            }
        }

        let out_stream = ReceiverStream::new(rx);

        Ok(tonic::Response::new(
            Box::pin(out_stream) as ProxyConnectionStream
        ))
    }
}
