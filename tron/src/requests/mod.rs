use crate::{Bridge, BridgeService, MessageStream, ServerSendMessageStream, ServerSendTitleStream};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::info;

pub mod balance;
pub mod friends;
pub mod leaderboards;
pub mod message;
pub mod players;
pub mod prefix;
pub mod servers;
pub mod session;
pub mod shop;
pub mod team;

#[tonic::async_trait]
impl Bridge for BridgeService {
    type ServerSendMessageStream = ServerSendMessageStream;
    type ServerSendTitleStream = ServerSendTitleStream;
    type MessageStream = MessageStream;

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

    async fn delete_team(
        &self,
        _request: Request<crate::bridge::DeleteTeamRequest>,
    ) -> Result<Response<crate::bridge::DeleteTeamResponse>, Status> {
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
        request: Request<crate::bridge::ServerSubscribeRequest>,
    ) -> Result<Response<Self::ServerSendMessageStream>, Status> {
        let client_id = request.into_inner().client_id;
        let (tx, rx) = mpsc::channel(32);

        self.state
            .send_message_clients
            .insert(client_id.clone(), tx);
        info!("✅ Client {client_id} connected to ServerSendMessage stream");

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ServerSendMessageStream
        ))
    }

    async fn server_send_title(
        &self,
        request: Request<crate::bridge::ServerSubscribeRequest>,
    ) -> Result<Response<Self::ServerSendTitleStream>, Status> {
        let client_id = request.into_inner().client_id;
        let (tx, rx) = mpsc::channel(32);

        self.state.send_title_clients.insert(client_id.clone(), tx);
        info!("✅ Client {client_id} connected to ServerSendTitle stream");

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ServerSendTitleStream
        ))
    }

    async fn message(
        &self,
        request: Request<tonic::Streaming<crate::bridge::MessageRequest>>,
    ) -> Result<Response<Self::MessageStream>, Status> {
        self.handle_message(request).await
    }

    // Prefixes

    async fn get_all_prefix(
        &self,
        request: Request<crate::bridge::GetAllPrefixRequest>,
    ) -> Result<Response<crate::bridge::GetAllPrefixResponse>, Status> {
        self.handle_get_all_prefixes(request).await
    }

    async fn buy_prefix(
        &self,
        request: Request<crate::bridge::BuyPrefixRequest>,
    ) -> Result<Response<crate::bridge::BuyPrefixResponse>, Status> {
        self.handle_buy_prefix(request).await
    }

    async fn get_owned_prefix(
        &self,
        request: Request<crate::bridge::GetOwnedPrefixRequest>,
    ) -> Result<Response<crate::bridge::GetOwnedPrefixResponse>, Status> {
        self.handle_get_owned_prefix(request).await
    }

    async fn delete_prefix(
        &self,
        request: Request<crate::bridge::DeletePrefixRequest>,
    ) -> Result<Response<crate::bridge::DeletePrefixResponse>, Status> {
        self.handle_delete_prefix(request).await
    }

    async fn create_prefix(
        &self,
        request: Request<crate::bridge::CreatePrefixRequest>,
    ) -> Result<Response<crate::bridge::CreatePrefixResponse>, Status> {
        self.handle_create_prefix(request).await
    }

    async fn select_prefix(
        &self,
        request: Request<crate::bridge::SelectPrefixRequest>,
    ) -> Result<Response<crate::bridge::SelectPrefixResponse>, Status> {
        self.handle_select_prefix(request).await
    }

    async fn get_current_prefix(
        &self,
        _request: Request<crate::bridge::GetCurrentPrefixRequest>,
    ) -> Result<Response<crate::bridge::GetCurrentPrefixResponse>, Status> {
        self.handle_get_current_prefix(_request).await
    }
}
