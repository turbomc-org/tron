use crate::cache::Cache;
use crate::collections::Collections;
use bridge::bridge_server::Bridge;
use futures::Stream;
use once_cell::sync::Lazy;
use snowflaked::sync::Generator;
use std::iter::Take;
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::info;

pub mod cache;
pub mod collections;
pub mod grpc;
pub mod logger;
pub mod models;
pub mod modules;
pub mod requests;
pub mod utils;

type ServerSendMessageStream = Pin<
    Box<
        dyn Stream<Item = Result<crate::bridge::ServerSendMessageResponse, Status>>
            + Send
            + 'static,
    >,
>;

static RETRY_STRATEGY: Lazy<Take<ExponentialBackoff>> = Lazy::new(|| {
    ExponentialBackoff::from_millis(100)
        .max_delay(Duration::from_secs(1))
        .take(3)
});

static GENERATOR: Generator = Generator::new(0);

pub mod bridge {
    tonic::include_proto!("bridge");
}

pub struct BridgeService {
    cache: Cache,
    collections: Collections,
}

impl BridgeService {
    pub async fn new(collections: Collections) -> Self {
        let cache = Cache::init(&collections)
            .await
            .expect("failed to load cache");

        Self { cache, collections }
    }

    pub async fn broadcast_message(&self, msg: bridge::ServerSendMessageResponse) {
        let mut dead_clients = Vec::new();

        for entry in self.cache.clients.iter() {
            let client_id = *entry.key();
            let tx = entry.value();

            if tx.send(Ok(msg.clone())).await.is_err() {
                dead_clients.push(client_id);
            }
        }

        for id in dead_clients {
            self.cache.clients.remove(&id);
        }
    }

    pub async fn send_to_client(
        &self,
        client_id: u64,
        msg: bridge::ServerSendMessageResponse,
    ) -> Result<(), Status> {
        if let Some(entry) = self.cache.clients.get(&client_id) {
            let tx = entry.value();
            tx.send(Ok(msg))
                .await
                .map_err(|_| Status::unavailable("Client disconnected"))?;
            Ok(())
        } else {
            Err(Status::not_found("Client not found"))
        }
    }
}

#[tonic::async_trait]
impl Bridge for BridgeService {
    type ServerSendMessageStream = ServerSendMessageStream;

    async fn player_join(
        &self,
        request: Request<crate::bridge::PlayerJoinRequest>,
    ) -> Result<Response<crate::bridge::PlayerJoinResponse>, Status> {
        self.handle_player_join(request).await
    }

    async fn player_leave(
        &self,
        request: Request<crate::bridge::PlayerLeaveRequest>,
    ) -> Result<Response<crate::bridge::PlayerLeaveResponse>, Status> {
        self.handle_player_leave(request).await
    }

    async fn player_place_block(
        &self,
        _request: Request<crate::bridge::PlayerPlaceBlockRequest>,
    ) -> Result<Response<crate::bridge::PlayerPlaceBlockResponse>, Status> {
        unimplemented!()
    }

    async fn player_break_block(
        &self,
        _request: Request<crate::bridge::PlayerBreakBlockRequest>,
    ) -> Result<Response<crate::bridge::PlayerBreakBlockResponse>, Status> {
        unimplemented!()
    }

    async fn get_balance(
        &self,
        request: Request<crate::bridge::GetBalanceRequest>,
    ) -> Result<Response<crate::bridge::GetBalanceResponse>, Status> {
        self.handle_get_balance(request).await
    }

    async fn transfer_balance(
        &self,
        request: Request<crate::bridge::TransferBalanceRequest>,
    ) -> Result<Response<crate::bridge::TransferBalanceResponse>, Status> {
        self.handle_transfer_balance(request).await
    }

    async fn overall_leaderboard(
        &self,
        request: Request<crate::bridge::OverallLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::OverallLeaderboardResponse>, Status> {
        self.handle_overall_leaderboard(request).await
    }

    async fn coins_leaderboard(
        &self,
        request: Request<crate::bridge::CoinsLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::CoinsLeaderboardResponse>, Status> {
        self.handle_coins_leaderboard(request).await
    }

    async fn kda_leaderboard(
        &self,
        request: Request<crate::bridge::KdaLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::KdaLeaderboardResponse>, Status> {
        self.handle_kda_leaderboard(request).await
    }

    async fn kills_leaderboard(
        &self,
        request: Request<crate::bridge::KillsLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::KillsLeaderboardResponse>, Status> {
        self.handle_kills_leaderboard(request).await
    }

    async fn deaths_leaderboard(
        &self,
        request: Request<crate::bridge::DeathsLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::DeathsLeaderboardResponse>, Status> {
        self.handle_deaths_leaderboard(request).await
    }

    async fn teams_leaderboard(
        &self,
        request: Request<crate::bridge::TeamsLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::TeamsLeaderboardResponse>, Status> {
        self.handle_teams_leaderboard(request).await
    }

    async fn send_message(
        &self,
        request: Request<crate::bridge::SendMessageRequest>,
    ) -> Result<Response<crate::bridge::SendMessageResponse>, Status> {
        self.handle_send_message(request).await
    }

    async fn get_friends(
        &self,
        request: Request<crate::bridge::GetFriendsRequest>,
    ) -> Result<Response<crate::bridge::GetFriendsResponse>, Status> {
        self.handle_get_friends(request).await
    }

    async fn send_friend_request(
        &self,
        request: Request<crate::bridge::SendFriendRequestRequest>,
    ) -> Result<Response<crate::bridge::SendFriendRequestResponse>, Status> {
        self.handle_send_friend_request(request).await
    }

    async fn accept_friend_request(
        &self,
        request: Request<crate::bridge::AcceptFriendRequestRequest>,
    ) -> Result<Response<crate::bridge::AcceptFriendRequestResponse>, Status> {
        self.handle_accept_friend_request(request).await
    }

    async fn reject_friend_request(
        &self,
        request: Request<crate::bridge::RejectFriendRequestRequest>,
    ) -> Result<Response<crate::bridge::RejectFriendRequestResponse>, Status> {
        self.handle_reject_friend_request(request).await
    }

    async fn get_friend_requests(
        &self,
        request: Request<crate::bridge::GetFriendRequestsRequest>,
    ) -> Result<Response<crate::bridge::GetFriendRequestsResponse>, Status> {
        self.handle_get_friend_requests(request).await
    }

    async fn remove_friend(
        &self,
        request: Request<crate::bridge::RemoveFriendRequest>,
    ) -> Result<Response<crate::bridge::RemoveFriendResponse>, Status> {
        self.handle_remove_friend(request).await
    }

    async fn create_team(
        &self,
        request: Request<crate::bridge::CreateTeamRequest>,
    ) -> Result<Response<crate::bridge::CreateTeamResponse>, Status> {
        self.handle_create_team(request).await
    }

    async fn leave_team(
        &self,
        request: Request<crate::bridge::LeaveTeamRequest>,
    ) -> Result<Response<crate::bridge::LeaveTeamResponse>, Status> {
        self.handle_leave_team(request).await
    }

    async fn join_team(
        &self,
        request: Request<crate::bridge::JoinTeamRequest>,
    ) -> Result<Response<crate::bridge::JoinTeamResponse>, Status> {
        self.handle_join_team(request).await
    }

    async fn send_team_invite(
        &self,
        request: Request<crate::bridge::SendTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::SendTeamInviteResponse>, Status> {
        self.handle_send_team_invite(request).await
    }

    async fn accept_team_invite(
        &self,
        request: Request<crate::bridge::AcceptTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::AcceptTeamInviteResponse>, Status> {
        self.handle_accept_team_invite(request).await
    }

    async fn reject_team_invite(
        &self,
        request: Request<crate::bridge::RejectTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::RejectTeamInviteResponse>, Status> {
        self.handle_reject_team_invite(request).await
    }

    async fn get_team_members(
        &self,
        request: Request<crate::bridge::GetTeamMembersRequest>,
    ) -> Result<Response<crate::bridge::GetTeamMembersResponse>, Status> {
        self.handle_get_team_members(request).await
    }

    async fn remove_team_member(
        &self,
        request: Request<crate::bridge::RemoveTeamMemberRequest>,
    ) -> Result<Response<crate::bridge::RemoveTeamMemberResponse>, Status> {
        self.handle_remove_team_member(request).await
    }

    async fn promote_team_member(
        &self,
        request: Request<crate::bridge::PromoteTeamMemberRequest>,
    ) -> Result<Response<crate::bridge::PromoteTeamMemberResponse>, Status> {
        self.handle_promote_team_member(request).await
    }

    async fn get_open_teams(
        &self,
        _request: Request<crate::bridge::GetOpenTeamsRequest>,
    ) -> Result<Response<crate::bridge::GetOpenTeamsResponse>, Status> {
        unimplemented!()
    }

    async fn buy_item(
        &self,
        request: Request<crate::bridge::BuyItemRequest>,
    ) -> Result<Response<crate::bridge::BuyItemResponse>, Status> {
        self.handle_buy_item(request).await
    }

    async fn sell_item(
        &self,
        request: Request<crate::bridge::SellItemRequest>,
    ) -> Result<Response<crate::bridge::SellItemResponse>, Status> {
        self.handle_sell_item(request).await
    }

    async fn get_items(
        &self,
        _request: Request<crate::bridge::GetItemsRequest>,
    ) -> Result<Response<crate::bridge::GetItemsResponse>, Status> {
        unimplemented!()
    }

    async fn player_death(
        &self,
        request: Request<crate::bridge::PlayerDeathRequest>,
    ) -> Result<Response<crate::bridge::PlayerDeathResponse>, Status> {
        self.handle_player_death(request).await
    }

    async fn player_kill(
        &self,
        request: Request<crate::bridge::PlayerKillRequest>,
    ) -> Result<Response<crate::bridge::PlayerKillResponse>, Status> {
        self.handle_player_kill(request).await
    }

    async fn proxy_startup(
        &self,
        _request: Request<crate::bridge::ProxyStartupRequest>,
    ) -> Result<Response<crate::bridge::ProxyStartupResponse>, Status> {
        unimplemented!()
    }

    async fn proxy_shutdown(
        &self,
        _request: Request<crate::bridge::ProxyShutdownRequest>,
    ) -> Result<Response<crate::bridge::ProxyShutdownResponse>, Status> {
        unimplemented!()
    }

    async fn survival_startup(
        &self,
        _request: Request<crate::bridge::SurvivalStartupRequest>,
    ) -> Result<Response<crate::bridge::SurvivalStartupResponse>, Status> {
        unimplemented!()
    }

    async fn survival_shutdown(
        &self,
        _request: Request<crate::bridge::SurvivalShutdownRequest>,
    ) -> Result<Response<crate::bridge::SurvivalShutdownResponse>, Status> {
        unimplemented!()
    }

    async fn lobby_startup(
        &self,
        _request: Request<crate::bridge::LobbyStartupRequest>,
    ) -> Result<Response<crate::bridge::LobbyStartupResponse>, Status> {
        unimplemented!()
    }

    async fn lobby_shutdown(
        &self,
        _request: Request<crate::bridge::LobbyShutdownRequest>,
    ) -> Result<Response<crate::bridge::LobbyShutdownResponse>, Status> {
        unimplemented!()
    }

    async fn report_player(
        &self,
        _request: Request<crate::bridge::ReportPlayerRequest>,
    ) -> Result<Response<crate::bridge::ReportPlayerResponse>, Status> {
        unimplemented!()
    }

    async fn server_send_message(
        &self,
        request: Request<crate::bridge::ServerSendMessageRequest>,
    ) -> Result<Response<Self::ServerSendMessageStream>, Status> {
        let client_id = request.into_inner().client_id;
        let (tx, rx) = mpsc::channel(32);

        self.cache.clients.insert(client_id.clone(), tx);
        info!("✅ Client {client_id} connected to ServerSendMessage stream");

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ServerSendMessageStream
        ))
    }

    // Prefixes

    async fn get_all_prefix(
        &self,
        _request: Request<crate::bridge::GetAllPrefixRequest>,
    ) -> Result<Response<crate::bridge::GetAllPrefixResponse>, Status> {
        unimplemented!()
    }

    async fn buy_prefix(
        &self,
        _request: Request<crate::bridge::BuyPrefixRequest>,
    ) -> Result<Response<crate::bridge::BuyPrefixResponse>, Status> {
        unimplemented!()
    }

    async fn get_owned_prefix(
        &self,
        _request: Request<crate::bridge::GetOwnedPrefixRequest>,
    ) -> Result<Response<crate::bridge::GetOwnedPrefixResponse>, Status> {
        unimplemented!()
    }

    async fn delete_prefix(
        &self,
        _request: Request<crate::bridge::DeletePrefixRequest>,
    ) -> Result<Response<crate::bridge::DeletePrefixResponse>, Status> {
        unimplemented!()
    }

    async fn create_prefix(
        &self,
        _request: Request<crate::bridge::CreatePrefixRequest>,
    ) -> Result<Response<crate::bridge::CreatePrefixResponse>, Status> {
        unimplemented!()
    }
}
