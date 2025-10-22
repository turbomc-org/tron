use crate::cache::Cache;
use crate::models::databases::Databases;
use crate::utils::mongodb::MongoDB;
use crate::utils::redis::Redis;
use bridge::bridge_server::Bridge;
use snowflaked::sync::Generator;
use tonic::{Request, Response, Status};

pub mod cache;
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
    #[allow(unused)]
    mongodb: MongoDB,
    databases: Databases,
    #[allow(unused)]
    redis: Redis,
}

impl BridgeService {
    pub async fn new() -> Self {
        let mongodb = MongoDB::new().await.expect("failed to connect to MongoDB");
        let databases = Databases::new(&mongodb.database);
        let cache = Cache::init(&databases).await.expect("failed to load cache");
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

    async fn get_team(
        &self,
        request: Request<crate::bridge::GetTeamRequest>,
    ) -> Result<Response<crate::bridge::GetTeamResponse>, Status> {
        self.handle_get_team(request).await
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

    async fn buy_item(
        &self,
        _request: Request<crate::bridge::BuyItemRequest>,
    ) -> Result<Response<crate::bridge::BuyItemResponse>, Status> {
        unimplemented!()
    }

    async fn sell_item(
        &self,
        _request: Request<crate::bridge::SellItemRequest>,
    ) -> Result<Response<crate::bridge::SellItemResponse>, Status> {
        unimplemented!()
    }

    async fn get_items(
        &self,
        _request: Request<crate::bridge::GetItemsRequest>,
    ) -> Result<Response<crate::bridge::GetItemsResponse>, Status> {
        unimplemented!()
    }
}
