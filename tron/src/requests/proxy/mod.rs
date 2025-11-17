use crate::bridge::ProxyConnectionRequest;
use crate::{BridgeService, ProxyConnectionStream};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::error;
use tracing::info;

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
        let this = self.clone();

        tokio::spawn(async move {
            if let Some(Ok(first_msg)) = in_stream.next().await {
                if let Some(crate::bridge::proxy_connection_request::Payload::Handshake(
                    handshake,
                )) = first_msg.payload
                {
                    let proxy_id = handshake.proxy_id;
                    info!(
                        "🤝 Received handshake from Proxy ID: {}. Connection is now live.",
                        proxy_id
                    );

                    state.proxy_connections.insert(proxy_id, tx);

                    while let Some(result) = in_stream.next().await {
                        match result {
                            Ok(v) => {
                                if let Some(payload) = v.payload {
                                    match payload {
                                        crate::bridge::proxy_connection_request::Payload::Heartbeat(p) => {
                                            this.handle_proxy_heartbeat(p).await;
                                        }
                                        crate::bridge::proxy_connection_request::Payload::EmitMessage(p) => {
                                            this.handle_proxy_emit_message(p).await;
                                        }
                                        crate::bridge::proxy_connection_request::Payload::EmitWhisper(p) => {
                                            this.handle_proxy_emit_whisper(p).await;
                                        }
                                        crate::bridge::proxy_connection_request::Payload::Handshake(_) => {
                                            error!("Proxy {} sent a second handshake.", proxy_id);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Error in stream from proxy {}: {}", proxy_id, err);
                                break;
                            }
                        }
                    }

                    info!("Proxy {} disconnected.", proxy_id);
                    state.proxy_connections.remove(&proxy_id);
                } else {
                    error!("Proxy connection failed: First message was not a handshake.");
                }
            } else {
                info!("Proxy connected but sent no data.");
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(
            Box::pin(out_stream) as ProxyConnectionStream
        ))
    }
}
