use crate::bridge::*;
use crate::{Bridge, BridgeService, ProxyConnectionStream};
use tonic::{Request, Response, Status};

pub mod balance;
pub mod bug;
pub mod chat;
pub mod friends;
pub mod leaderboards;
pub mod perms;
pub mod players;
pub mod prefix;
pub mod proxy;
pub mod redeem;
pub mod report;
pub mod scoreboard;
pub mod servers;
pub mod session;
pub mod shop;
pub mod team;

#[tonic::async_trait]
impl Bridge for BridgeService {
    type ProxyConnectionStream = ProxyConnectionStream;

    // Session management

    // Servers
    async fn get_all_servers(
        &self,
        request: Request<GetAllServersRequest>,
    ) -> Result<Response<GetAllServersResponse>, Status> {
        self.handle_get_all_servers(request).await
    }

    async fn list_all_servers(
        &self,
        request: Request<ListAllServersRequest>,
    ) -> Result<Response<ListAllServersResponse>, Status> {
        self.handle_list_all_servers(request).await
    }

    async fn get_server(
        &self,
        request: Request<GetServerRequest>,
    ) -> Result<Response<GetServerResponse>, Status> {
        self.handle_get_server(request).await
    }

    async fn view_server(
        &self,
        request: Request<ViewServerRequest>,
    ) -> Result<Response<ViewServerResponse>, Status> {
        self.handle_view_server(request).await
    }

    async fn create_server(
        &self,
        request: Request<CreateServerRequest>,
    ) -> Result<Response<CreateServerResponse>, Status> {
        self.handle_create_server(request).await
    }

    async fn delete_server(
        &self,
        request: Request<DeleteServerRequest>,
    ) -> Result<Response<DeleteServerResponse>, Status> {
        self.handle_delete_server(request).await
    }

    async fn player_post_login(
        &self,
        request: Request<PlayerPostLoginRequest>,
    ) -> Result<Response<PlayerPostLoginResponse>, Status> {
        self.handle_player_post_login(request).await
    }

    async fn player_pre_login(
        &self,
        request: Request<PlayerPreLoginRequest>,
    ) -> Result<Response<PlayerPreLoginResponse>, Status> {
        self.handle_player_pre_login(request).await
    }

    async fn player_password_login(
        &self,
        request: Request<PlayerPasswordLoginRequest>,
    ) -> Result<Response<PlayerPasswordLoginResponse>, Status> {
        self.handle_player_password_login(request).await
    }

    async fn player_encryption_login(
        &self,
        request: Request<PlayerEncryptionLoginRequest>,
    ) -> Result<Response<PlayerEncryptionLoginResponse>, Status> {
        self.handle_player_encryption_login(request).await
    }

    async fn player_floodgate_login(
        &self,
        request: Request<PlayerFloodgateLoginRequest>,
    ) -> Result<Response<PlayerFloodgateLoginResponse>, Status> {
        self.handle_player_floodgate_login(request).await
    }

    async fn player_logout(
        &self,
        request: Request<PlayerLogoutRequest>,
    ) -> Result<Response<PlayerLogoutResponse>, Status> {
        self.handle_player_logout(request).await
    }

    async fn player_place_block(
        &self,
        request: Request<PlayerPlaceBlockRequest>,
    ) -> Result<Response<PlayerPlaceBlockResponse>, Status> {
        self.handle_player_place_block(request).await
    }

    async fn player_break_block(
        &self,
        request: Request<PlayerBreakBlockRequest>,
    ) -> Result<Response<PlayerBreakBlockResponse>, Status> {
        self.handle_player_break_block(request).await
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        self.handle_get_balance(request).await
    }

    async fn transfer_balance(
        &self,
        request: Request<TransferBalanceRequest>,
    ) -> Result<Response<TransferBalanceResponse>, Status> {
        self.handle_transfer_balance(request).await
    }

    // Leaderboards

    async fn overall_leaderboard(
        &self,
        request: Request<OverallLeaderboardRequest>,
    ) -> Result<Response<OverallLeaderboardResponse>, Status> {
        self.handle_overall_leaderboard(request).await
    }

    async fn coins_leaderboard(
        &self,
        request: Request<CoinsLeaderboardRequest>,
    ) -> Result<Response<CoinsLeaderboardResponse>, Status> {
        self.handle_coins_leaderboard(request).await
    }

    async fn kda_leaderboard(
        &self,
        request: Request<KdaLeaderboardRequest>,
    ) -> Result<Response<KdaLeaderboardResponse>, Status> {
        self.handle_kda_leaderboard(request).await
    }

    async fn kills_leaderboard(
        &self,
        request: Request<KillsLeaderboardRequest>,
    ) -> Result<Response<KillsLeaderboardResponse>, Status> {
        self.handle_kills_leaderboard(request).await
    }

    async fn deaths_leaderboard(
        &self,
        request: Request<DeathsLeaderboardRequest>,
    ) -> Result<Response<DeathsLeaderboardResponse>, Status> {
        self.handle_deaths_leaderboard(request).await
    }

    async fn teams_leaderboard(
        &self,
        request: Request<TeamsLeaderboardRequest>,
    ) -> Result<Response<TeamsLeaderboardResponse>, Status> {
        self.handle_teams_leaderboard(request).await
    }

    async fn list_overall_leaderboard(
        &self,
        request: Request<ListOverallLeaderboardRequest>,
    ) -> Result<Response<ListOverallLeaderboardResponse>, Status> {
        self.handle_list_overall_leaderboard(request).await
    }

    async fn list_kda_leaderboard(
        &self,
        request: Request<ListKdaLeaderboardRequest>,
    ) -> Result<Response<ListKdaLeaderboardResponse>, Status> {
        self.handle_list_kda_leaderboard(request).await
    }

    async fn list_kills_leaderboard(
        &self,
        request: Request<ListKillsLeaderboardRequest>,
    ) -> Result<Response<ListKillsLeaderboardResponse>, Status> {
        self.handle_list_kills_leaderboard(request).await
    }

    async fn list_deaths_leaderboard(
        &self,
        request: Request<ListDeathsLeaderboardRequest>,
    ) -> Result<Response<ListDeathsLeaderboardResponse>, Status> {
        self.handle_list_deaths_leaderboard(request).await
    }

    async fn list_coins_leaderboard(
        &self,
        request: Request<ListCoinsLeaderboardRequest>,
    ) -> Result<Response<ListCoinsLeaderboardResponse>, Status> {
        self.handle_list_coins_leaderboard(request).await
    }

    async fn list_teams_leaderboard(
        &self,
        request: Request<ListTeamsLeaderboardRequest>,
    ) -> Result<Response<ListTeamsLeaderboardResponse>, Status> {
        self.handle_list_teams_leaderboard(request).await
    }

    async fn get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        self.handle_get_friends(request).await
    }

    async fn send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        self.handle_send_friend_request(request).await
    }

    async fn accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        self.handle_accept_friend_request(request).await
    }

    async fn reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        self.handle_reject_friend_request(request).await
    }

    async fn get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        self.handle_get_friend_requests(request).await
    }

    async fn remove_friend(
        &self,
        request: Request<RemoveFriendRequest>,
    ) -> Result<Response<RemoveFriendResponse>, Status> {
        self.handle_remove_friend(request).await
    }

    async fn create_team(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        self.handle_create_team(request).await
    }

    async fn leave_team(
        &self,
        request: Request<LeaveTeamRequest>,
    ) -> Result<Response<LeaveTeamResponse>, Status> {
        self.handle_leave_team(request).await
    }

    async fn join_team(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        self.handle_join_team(request).await
    }

    async fn send_team_invite(
        &self,
        request: Request<SendTeamInviteRequest>,
    ) -> Result<Response<SendTeamInviteResponse>, Status> {
        self.handle_send_team_invite(request).await
    }

    async fn accept_team_invite(
        &self,
        request: Request<AcceptTeamInviteRequest>,
    ) -> Result<Response<AcceptTeamInviteResponse>, Status> {
        self.handle_accept_team_invite(request).await
    }

    async fn reject_team_invite(
        &self,
        request: Request<RejectTeamInviteRequest>,
    ) -> Result<Response<RejectTeamInviteResponse>, Status> {
        self.handle_reject_team_invite(request).await
    }

    async fn get_team_members(
        &self,
        request: Request<GetTeamMembersRequest>,
    ) -> Result<Response<GetTeamMembersResponse>, Status> {
        self.handle_get_team_members(request).await
    }

    async fn remove_team_member(
        &self,
        request: Request<RemoveTeamMemberRequest>,
    ) -> Result<Response<RemoveTeamMemberResponse>, Status> {
        self.handle_remove_team_member(request).await
    }

    async fn promote_team_member(
        &self,
        request: Request<PromoteTeamMemberRequest>,
    ) -> Result<Response<PromoteTeamMemberResponse>, Status> {
        self.handle_promote_team_member(request).await
    }

    async fn get_open_teams(
        &self,
        request: Request<GetOpenTeamsRequest>,
    ) -> Result<Response<GetOpenTeamsResponse>, Status> {
        self.handle_get_open_teams(request).await
    }

    async fn delete_team(
        &self,
        request: Request<DeleteTeamRequest>,
    ) -> Result<Response<DeleteTeamResponse>, Status> {
        self.handle_delete_team(request).await
    }

    // Shop

    async fn list_all_items(
        &self,
        request: Request<ListAllItemsRequest>,
    ) -> Result<Response<ListAllItemsResponse>, Status> {
        self.handle_list_all_items(request).await
    }

    async fn view_item(
        &self,
        request: Request<ViewItemRequest>,
    ) -> Result<Response<ViewItemResponse>, Status> {
        self.handle_view_item(request).await
    }

    async fn buy_item(
        &self,
        request: Request<BuyItemRequest>,
    ) -> Result<Response<BuyItemResponse>, Status> {
        self.handle_buy_item(request).await
    }

    async fn sell_item(
        &self,
        request: Request<SellItemRequest>,
    ) -> Result<Response<SellItemResponse>, Status> {
        self.handle_sell_item(request).await
    }

    async fn get_all_items(
        &self,
        request: Request<GetAllItemsRequest>,
    ) -> Result<Response<GetAllItemsResponse>, Status> {
        self.handle_get_all_items(request).await
    }

    async fn create_shop_item(
        &self,
        request: Request<CreateShopItemRequest>,
    ) -> Result<Response<CreateShopItemResponse>, Status> {
        self.handle_create_shop_item(request).await
    }

    async fn delete_shop_item(
        &self,
        request: Request<DeleteShopItemRequest>,
    ) -> Result<Response<DeleteShopItemResponse>, Status> {
        self.handle_delete_item(request).await
    }

    // Player

    async fn player_death(
        &self,
        request: Request<PlayerDeathRequest>,
    ) -> Result<Response<PlayerDeathResponse>, Status> {
        self.handle_player_death(request).await
    }

    async fn player_kill(
        &self,
        request: Request<PlayerKillRequest>,
    ) -> Result<Response<PlayerKillResponse>, Status> {
        self.handle_player_kill(request).await
    }

    async fn proxy_startup(
        &self,
        request: Request<ProxyStartupRequest>,
    ) -> Result<Response<ProxyStartupResponse>, Status> {
        self.handle_proxy_startup(request).await
    }

    async fn proxy_shutdown(
        &self,
        request: Request<ProxyShutdownRequest>,
    ) -> Result<Response<ProxyShutdownResponse>, Status> {
        self.handle_proxy_shutdown(request).await
    }

    async fn survival_startup(
        &self,
        request: Request<SurvivalStartupRequest>,
    ) -> Result<Response<SurvivalStartupResponse>, Status> {
        self.handle_survival_startup(request).await
    }

    async fn survival_shutdown(
        &self,
        request: Request<SurvivalShutdownRequest>,
    ) -> Result<Response<SurvivalShutdownResponse>, Status> {
        self.handle_survival_shutdown(request).await
    }

    async fn lobby_startup(
        &self,
        request: Request<LobbyStartupRequest>,
    ) -> Result<Response<LobbyStartupResponse>, Status> {
        self.handle_lobby_startup(request).await
    }

    async fn lobby_shutdown(
        &self,
        request: Request<LobbyShutdownRequest>,
    ) -> Result<Response<LobbyShutdownResponse>, Status> {
        self.handle_lobby_shutdown(request).await
    }

    // Reports

    async fn get_report(
        &self,
        request: Request<GetReportRequest>,
    ) -> Result<Response<GetReportResponse>, Status> {
        self.handle_get_report(request).await
    }

    async fn get_all_reports(
        &self,
        request: Request<GetAllReportsRequest>,
    ) -> Result<Response<GetAllReportsResponse>, Status> {
        self.handle_get_all_reports(request).await
    }

    async fn list_all_reports(
        &self,
        request: Request<ListAllReportsRequest>,
    ) -> Result<Response<ListAllReportsResponse>, Status> {
        self.handle_list_all_reports(request).await
    }

    async fn view_report(
        &self,
        request: Request<ViewReportRequest>,
    ) -> Result<Response<ViewReportResponse>, Status> {
        self.handle_view_report(request).await
    }

    async fn report_player(
        &self,
        request: Request<ReportPlayerRequest>,
    ) -> Result<Response<ReportPlayerResponse>, Status> {
        self.handle_report_player(request).await
    }

    async fn delete_report(
        &self,
        request: Request<DeleteReportRequest>,
    ) -> Result<Response<DeleteReportResponse>, Status> {
        self.handle_delete_report(request).await
    }

    async fn proxy_connection(
        &self,
        request: tonic::Request<tonic::Streaming<ProxyConnectionRequest>>,
    ) -> Result<tonic::Response<ProxyConnectionStream>, Status> {
        self.handle_proxy_connection(request).await
    }

    // Prefixes

    async fn get_all_prefix(
        &self,
        request: Request<GetAllPrefixRequest>,
    ) -> Result<Response<GetAllPrefixResponse>, Status> {
        self.handle_get_all_prefixes(request).await
    }

    async fn buy_prefix(
        &self,
        request: Request<BuyPrefixRequest>,
    ) -> Result<Response<BuyPrefixResponse>, Status> {
        self.handle_buy_prefix(request).await
    }

    async fn get_owned_prefix(
        &self,
        request: Request<GetOwnedPrefixRequest>,
    ) -> Result<Response<GetOwnedPrefixResponse>, Status> {
        self.handle_get_owned_prefix(request).await
    }

    async fn delete_prefix(
        &self,
        request: Request<DeletePrefixRequest>,
    ) -> Result<Response<DeletePrefixResponse>, Status> {
        self.handle_delete_prefix(request).await
    }

    async fn create_prefix(
        &self,
        request: Request<CreatePrefixRequest>,
    ) -> Result<Response<CreatePrefixResponse>, Status> {
        self.handle_create_prefix(request).await
    }

    async fn equip_prefix(
        &self,
        request: Request<EquipPrefixRequest>,
    ) -> Result<Response<EquipPrefixResponse>, Status> {
        self.handle_equip_prefix(request).await
    }

    async fn get_current_prefix(
        &self,
        request: Request<GetCurrentPrefixRequest>,
    ) -> Result<Response<GetCurrentPrefixResponse>, Status> {
        self.handle_get_current_prefix(request).await
    }

    async fn list_friends(
        &self,
        request: Request<ListFriendsRequest>,
    ) -> Result<Response<ListFriendsResponse>, Status> {
        self.handle_list_friends(request).await
    }

    async fn list_friend_requests(
        &self,
        request: Request<ListFriendRequestsRequest>,
    ) -> Result<Response<ListFriendRequestsResponse>, Status> {
        self.handle_list_friend_requests(request).await
    }

    async fn list_all_prefix(
        &self,
        request: Request<ListAllPrefixRequest>,
    ) -> Result<Response<ListAllPrefixResponse>, Status> {
        self.handle_list_all_prefix(request).await
    }

    async fn list_owned_prefix(
        &self,
        request: Request<ListOwnedPrefixRequest>,
    ) -> Result<Response<ListOwnedPrefixResponse>, Status> {
        self.handle_list_owned_prefix(request).await
    }

    async fn un_equip_prefix(
        &self,
        request: Request<UnEquipPrefixRequest>,
    ) -> Result<Response<UnEquipPrefixResponse>, Status> {
        self.handle_un_equip_prefix(request).await
    }

    async fn get_all_teams(
        &self,
        request: Request<GetAllTeamsRequest>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        self.handle_get_all_teams(request).await
    }

    async fn increase_coins(
        &self,
        request: Request<IncreaseCoinsRequest>,
    ) -> Result<Response<IncreaseCoinsResponse>, Status> {
        self.handle_increase_coins(request).await
    }

    // RedeemCode

    async fn get_all_redeem_codes(
        &self,
        request: Request<GetAllRedeemCodesRequest>,
    ) -> Result<Response<GetAllRedeemCodesResponse>, Status> {
        self.handle_get_all_redeem_codes(request).await
    }

    async fn list_all_redeem_codes(
        &self,
        request: Request<ListAllRedeemCodesRequest>,
    ) -> Result<Response<ListAllRedeemCodesResponse>, Status> {
        self.handle_list_all_redeem_codes(request).await
    }

    async fn redeem_code(
        &self,
        request: Request<RedeemCodeRequest>,
    ) -> Result<Response<RedeemCodeResponse>, Status> {
        self.handle_redeem_code(request).await
    }

    async fn create_redeem_code(
        &self,
        request: Request<CreateRedeemCodeRequest>,
    ) -> Result<Response<CreateRedeemCodeResponse>, Status> {
        self.handle_create_redeem_code(request).await
    }

    async fn delete_redeem_code(
        &self,
        request: Request<DeleteRedeemCodeRequest>,
    ) -> Result<Response<DeleteRedeemCodeResponse>, Status> {
        self.handle_delete_redeem_code(request).await
    }

    // Perms

    async fn promote_perms(
        &self,
        request: Request<PromotePermsRequest>,
    ) -> Result<Response<PromotePermsResponse>, Status> {
        self.handle_promote_perms(request).await
    }

    async fn demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        self.handle_demote_perms(request).await
    }

    async fn get_all_admins(
        &self,
        request: Request<GetAllAdminsRequest>,
    ) -> Result<Response<GetAllAdminsResponse>, Status> {
        self.handle_get_all_admins(request).await
    }

    async fn get_all_moderators(
        &self,
        request: Request<GetAllModeratorsRequest>,
    ) -> Result<Response<GetAllModeratorsResponse>, Status> {
        self.handle_get_all_moderators(request).await
    }

    async fn list_all_admins(
        &self,
        request: Request<ListAllAdminsRequest>,
    ) -> Result<Response<ListAllAdminsResponse>, Status> {
        self.handle_list_all_admins(request).await
    }

    async fn list_all_moderators(
        &self,
        request: Request<ListAllModeratorsRequest>,
    ) -> Result<Response<ListAllModeratorsResponse>, Status> {
        self.handle_list_all_moderators(request).await
    }

    // Chats / Messaging

    async fn exit_chat(
        &self,
        request: Request<ExitChatRequest>,
    ) -> Result<Response<ExitChatResponse>, Status> {
        self.handle_exit_chat(request).await
    }

    async fn global_chat(
        &self,
        request: Request<GlobalChatRequest>,
    ) -> Result<Response<GlobalChatResponse>, Status> {
        self.handle_global_chat(request).await
    }

    async fn hindi_chat(
        &self,
        request: Request<HindiChatRequest>,
    ) -> Result<Response<HindiChatResponse>, Status> {
        self.handle_hindi_chat(request).await
    }

    async fn friend_chat(
        &self,
        request: Request<FriendChatRequest>,
    ) -> Result<Response<FriendChatResponse>, Status> {
        self.handle_friend_chat(request).await
    }

    async fn team_chat(
        &self,
        request: Request<TeamChatRequest>,
    ) -> Result<Response<TeamChatResponse>, Status> {
        self.handle_team_chat(request).await
    }

    // Scoreboard

    async fn get_scoreboard(
        &self,
        request: Request<GetScoreboardRequest>,
    ) -> Result<Response<GetScoreboardResponse>, Status> {
        self.handle_get_scoreboard(request).await
    }

    async fn toggle_scoreboard(
        &self,
        request: Request<ToggleScoreboardRequest>,
    ) -> Result<Response<ToggleScoreboardResponse>, Status> {
        self.handle_toggle_scoreboard(request).await
    }

    // Bug

    async fn get_bug(
        &self,
        request: Request<GetBugRequest>,
    ) -> Result<Response<GetBugResponse>, Status> {
        self.handle_get_bug(request).await
    }

    async fn view_bug(
        &self,
        request: Request<ViewBugRequest>,
    ) -> Result<Response<ViewBugResponse>, Status> {
        self.handle_view_bug(request).await
    }

    async fn get_all_bugs(
        &self,
        request: Request<GetAllBugsRequest>,
    ) -> Result<Response<GetAllBugsResponse>, Status> {
        self.handle_get_all_bugs(request).await
    }

    async fn list_all_bugs(
        &self,
        request: Request<ListAllBugsRequest>,
    ) -> Result<Response<ListAllBugsResponse>, Status> {
        self.handle_list_all_bugs(request).await
    }

    async fn bug(&self, request: Request<BugRequest>) -> Result<Response<BugResponse>, Status> {
        self.handle_bug(request).await
    }

    async fn delete_bug(
        &self,
        request: Request<DeleteBugRequest>,
    ) -> Result<Response<DeleteBugResponse>, Status> {
        self.delete_bug(request).await
    }
}
