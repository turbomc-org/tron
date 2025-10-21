use crate::models::cache::Cache;
use crate::models::databases::Databases;
use crate::utils::mongodb::MongoDB;
use crate::utils::redis::Redis;
use bridge::bridge_server::Bridge;
use snowflaked::sync::Generator;
use tonic::{Request, Response, Status};

pub mod grpc;
pub mod logger;
pub mod models;
pub mod requests;
pub mod utils;

static GENERATOR: Generator = Generator::new(0);

pub mod bridge {
    tonic::include_proto!("bridge");
}

pub struct BridgeService {
    cache: Cache,
    mongodb: MongoDB,
    databases: Databases,
    redis: Redis,
}

impl BridgeService {
    pub async fn new() -> Self {
        let cache = Cache::new();
        let mongodb = MongoDB::new().await.expect("failed to connect to MongoDB");
        let databases = Databases::new(&mongodb.database);
        let redis = Redis::new();

        Self {
            cache,
            mongodb,
            databases,
            redis,
        }
    }
}

#[tonic::async_trait]
impl Bridge for BridgeService {
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

    async fn get_leaderboard(
        &self,
        request: Request<crate::bridge::GetLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::GetLeaderboardResponse>, Status> {
        self.handle_get_leaderboard(request).await
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

    async fn create_team(
        &self,
        request: Request<crate::bridge::CreateTeamRequest>,
    ) -> Result<Response<crate::bridge::CreateTeamResponse>, Status> {
        self.handle_create_team(request).await
    }

    async fn get_team(
        &self,
        request: Request<crate::bridge::GetTeamRequest>,
    ) -> Result<Response<crate::bridge::GetTeamResponse>, Status> {
        unimplemented!()
    }

    async fn get_team_leaderboard(
        &self,
        request: Request<crate::bridge::GetTeamLeaderboardRequest>,
    ) -> Result<Response<crate::bridge::GetTeamLeaderboardResponse>, Status> {
        unimplemented!()
    }

    async fn leave_team(
        &self,
        request: Request<crate::bridge::LeaveTeamRequest>,
    ) -> Result<Response<crate::bridge::LeaveTeamResponse>, Status> {
        unimplemented!()
    }

    async fn join_team(
        &self,
        request: Request<crate::bridge::JoinTeamRequest>,
    ) -> Result<Response<crate::bridge::JoinTeamResponse>, Status> {
        unimplemented!()
    }

    async fn send_team_invite(
        &self,
        request: Request<crate::bridge::SendTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::SendTeamInviteResponse>, Status> {
        unimplemented!()
    }

    async fn accept_team_invite(
        &self,
        request: Request<crate::bridge::AcceptTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::AcceptTeamInviteResponse>, Status> {
        unimplemented!()
    }

    async fn reject_team_invite(
        &self,
        request: Request<crate::bridge::RejectTeamInviteRequest>,
    ) -> Result<Response<crate::bridge::RejectTeamInviteResponse>, Status> {
        unimplemented!()
    }

    async fn get_team_members(
        &self,
        request: Request<crate::bridge::GetTeamMembersRequest>,
    ) -> Result<Response<crate::bridge::GetTeamMembersResponse>, Status> {
        unimplemented!()
    }

    async fn buy_item(
        &self,
        request: Request<crate::bridge::BuyItemRequest>,
    ) -> Result<Response<crate::bridge::BuyItemResponse>, Status> {
        unimplemented!()
    }

    async fn sell_item(
        &self,
        request: Request<crate::bridge::SellItemRequest>,
    ) -> Result<Response<crate::bridge::SellItemResponse>, Status> {
        unimplemented!()
    }

    async fn get_items(
        &self,
        request: Request<crate::bridge::GetItemsRequest>,
    ) -> Result<Response<crate::bridge::GetItemsResponse>, Status> {
        unimplemented!()
    }
}
