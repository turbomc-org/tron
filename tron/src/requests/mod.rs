use crate::bridge::*;
use crate::{Bridge, BridgeService, ProxyConnectionStream};
use tonic::{Request, Response, Status};

pub mod balance;
pub mod friends;
pub mod leaderboards;
pub mod players;
pub mod prefix;
pub mod proxy;
pub mod servers;
pub mod session;
pub mod shop;
pub mod team;

#[tonic::async_trait]
impl Bridge for BridgeService {
    type ProxyConnectionStream = ProxyConnectionStream;

    // Session management

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
        todo!("Implement overall list leaderboard")
    }

    async fn list_kda_leaderboard(
        &self,
        request: Request<ListKdaLeaderboardRequest>,
    ) -> Result<Response<ListKdaLeaderboardResponse>, Status> {
        todo!("Implement kda list leaderboard")
    }

    async fn list_kills_leaderboard(
        &self,
        request: Request<ListKillsLeaderboardRequest>,
    ) -> Result<Response<ListKillsLeaderboardResponse>, Status> {
        todo!("Implement kills list leaderboard")
    }

    async fn list_deaths_leaderboard(
        &self,
        request: Request<ListDeathsLeaderboardRequest>,
    ) -> Result<Response<ListDeathsLeaderboardResponse>, Status> {
        todo!("Implement deaths list leaderboard")
    }

    async fn list_coins_leaderboard(
        &self,
        request: Request<ListCoinsLeaderboardRequest>,
    ) -> Result<Response<ListCoinsLeaderboardResponse>, Status> {
        todo!("Implement coins list leaderboard")
    }

    async fn list_teams_leaderboard(
        &self,
        request: Request<ListTeamsLeaderboardRequest>,
    ) -> Result<Response<ListTeamsLeaderboardResponse>, Status> {
        todo!("Implement teams list leaderboard")
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

    async fn get_items(
        &self,
        _request: Request<GetItemsRequest>,
    ) -> Result<Response<GetItemsResponse>, Status> {
        todo!("Implement get shop items endpoint")
    }

    async fn create_shop_item(
        &self,
        _request: Request<CreateShopItemRequest>,
    ) -> Result<Response<CreateShopItemResponse>, Status> {
        todo!("Implement create shop item endpoint")
    }

    async fn delete_shop_item(
        &self,
        _request: Request<DeleteShopItemRequest>,
    ) -> Result<Response<DeleteShopItemResponse>, Status> {
        todo!("Implement delete shop item endpoint")
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

    async fn get_all_reports(
        &self,
        _request: Request<GetAllReportsRequest>,
    ) -> Result<Response<GetAllReportsResponse>, Status> {
        todo!("Implement get all reports")
    }

    async fn list_all_reports(
        &self,
        _request: Request<ListAllReportsRequest>,
    ) -> Result<Response<ListAllReportsResponse>, Status> {
        todo!("Implement list all reports")
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
        todo!("Implement delete report")
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
        todo!("Implement get all redeem codes")
    }

    async fn list_all_redeem_codes(
        &self,
        request: Request<ListRedeemCodesRequest>,
    ) -> Result<Response<ListRedeemCodesResponse>, Status> {
        todo!("Implement list all redeem codes")
    }

    async fn redeem_code(
        &self,
        request: Request<RedeemCodeRequest>,
    ) -> Result<Response<RedeemCodeResponse>, Status> {
        todo!("Implement redeem code")
    }

    async fn create_redeem_code(
        &self,
        request: Request<CreateRedeemCodeRequest>,
    ) -> Result<Response<CreateRedeemCodeResponse>, Status> {
        todo!("Implement create redeem code")
    }

    async fn delete_redeem_code(
        &self,
        request: Request<DeleteRedeemCodeRequest>,
    ) -> Result<Response<DeleteRedeemCodeResponse>, Status> {
        todo!("Implement delete redeem code")
    }

    // Perms

    async fn promote_perms(
        &self,
        request: Request<PromotePermsRequest>,
    ) -> Result<Response<PromotePermsResponse>, Status> {
        todo!("Implement promote player")
    }

    async fn demote_perms(
        &self,
        request: Request<DemotePermsRequest>,
    ) -> Result<Response<DemotePermsResponse>, Status> {
        todo!("Implement demote player")
    }

    async fn get_all_admins(
        &self,
        request: Request<GetAllAdminsRequest>,
    ) -> Result<Response<GetAllAdminsResponse>, Status> {
        todo!("Implement get all admins")
    }

    async fn get_all_moderators(
        &self,
        request: Request<GetAllModeratorsRequest>,
    ) -> Result<Response<GetAllModeratorsResponse>, Status> {
        todo!("Implement get all moderators")
    }

    async fn list_all_admins(
        &self,
        request: Request<ListAllAdminsRequest>,
    ) -> Result<Response<ListAllAdminsResponse>, Status> {
        todo!("Implement list all admins")
    }

    async fn list_all_moderators(
        &self,
        request: Request<ListAllModeratorsRequest>,
    ) -> Result<Response<ListAllModeratorsResponse>, Status> {
        todo!("Implement list all moderators")
    }

    // Chats / Messaging

    async fn exit_chat(
        &self,
        request: Request<ExitChatRequest>,
    ) -> Result<Response<ExitChatResponse>, Status> {
        todo!("Implement exit chat")
    }

    async fn global_chat(
        &self,
        request: Request<GlobalChatRequest>,
    ) -> Result<Response<GlobalChatResponse>, Status> {
        todo!("Implement global chat")
    }

    async fn hindi_chat(
        &self,
        request: Request<HindiChatRequest>,
    ) -> Result<Response<HindiChatResponse>, Status> {
        todo!("Implement hindi chat")
    }

    async fn friend_chat(
        &self,
        request: Request<FriendChatRequest>,
    ) -> Result<Response<FriendChatResponse>, Status> {
        todo!("Implement friend chat")
    }

    async fn team_chat(
        &self,
        request: Request<TeamChatRequest>,
    ) -> Result<Response<TeamChatResponse>, Status> {
        todo!("Implement team chat")
    }

    // Scoreboard

    async fn get_scoreboard(
        &self,
        request: Request<GetScoreboardRequest>,
    ) -> Result<Response<GetScoreboardResponse>, Status> {
        todo!("Implement get scoreboard")
    }

    async fn toggle_scoreboard(
        &self,
        request: Request<ToggleScoreboardRequest>,
    ) -> Result<Response<ToggleScoreboardResponse>, Status> {
        todo!("Implement toggle scoreboard")
    }

    // Bug
    async fn get_bug(
        &self,
        request: Request<GetBugRequest>,
    ) -> Result<Response<GetBugResponse>, Status> {
        todo!("Implement get bug")
    }

    async fn view_bug(
        &self,
        request: Request<ViewBugRequest>,
    ) -> Result<Response<ViewBugResponse>, Status> {
        todo!("Implement view bug")
    }

    async fn get_all_bugs(
        &self,
        request: Request<GetAllBugsRequest>,
    ) -> Result<Response<GetAllBugsResponse>, Status> {
        todo!("Implement get all bugs")
    }

    async fn list_all_bugs(
        &self,
        request: Request<ListAllBugsRequest>,
    ) -> Result<Response<ListAllBugsResponse>, Status> {
        todo!("Implement list all bugs")
    }

    async fn bug(&self, request: Request<BugRequest>) -> Result<Response<BugResponse>, Status> {
        todo!("Implement bug")
    }

    async fn delete_bug(
        &self,
        request: Request<DeleteBugRequest>,
    ) -> Result<Response<DeleteBugResponse>, Status> {
        todo!("Implement delete bug")
    }
}
