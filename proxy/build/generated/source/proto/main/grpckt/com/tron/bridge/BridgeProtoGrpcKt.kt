package com.tron.bridge

import com.tron.bridge.BridgeGrpc.getServiceDescriptor
import io.grpc.CallOptions
import io.grpc.CallOptions.DEFAULT
import io.grpc.Channel
import io.grpc.Metadata
import io.grpc.MethodDescriptor
import io.grpc.ServerServiceDefinition
import io.grpc.ServerServiceDefinition.builder
import io.grpc.ServiceDescriptor
import io.grpc.Status.UNIMPLEMENTED
import io.grpc.StatusException
import io.grpc.kotlin.AbstractCoroutineServerImpl
import io.grpc.kotlin.AbstractCoroutineStub
import io.grpc.kotlin.ClientCalls.serverStreamingRpc
import io.grpc.kotlin.ClientCalls.unaryRpc
import io.grpc.kotlin.ServerCalls.serverStreamingServerMethodDefinition
import io.grpc.kotlin.ServerCalls.unaryServerMethodDefinition
import io.grpc.kotlin.StubFor
import kotlin.String
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.jvm.JvmOverloads
import kotlin.jvm.JvmStatic
import kotlinx.coroutines.flow.Flow

/**
 * Holder for Kotlin coroutine-based client and server APIs for bridge.Bridge.
 */
public object BridgeGrpcKt {
  public const val SERVICE_NAME: String = BridgeGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = getServiceDescriptor()

  public val playerJoinMethod:
      MethodDescriptor<Session.PlayerJoinRequest, Session.PlayerJoinResponse>
    @JvmStatic
    get() = BridgeGrpc.getPlayerJoinMethod()

  public val playerLeaveMethod:
      MethodDescriptor<Session.PlayerLeaveRequest, Session.PlayerLeaveResponse>
    @JvmStatic
    get() = BridgeGrpc.getPlayerLeaveMethod()

  public val getBalanceMethod:
      MethodDescriptor<Balance.GetBalanceRequest, Balance.GetBalanceResponse>
    @JvmStatic
    get() = BridgeGrpc.getGetBalanceMethod()

  public val transferBalanceMethod:
      MethodDescriptor<Balance.TransferBalanceRequest, Balance.TransferBalanceResponse>
    @JvmStatic
    get() = BridgeGrpc.getTransferBalanceMethod()

  public val overallLeaderboardMethod:
      MethodDescriptor<Leaderboard.OverallLeaderboardRequest, Leaderboard.OverallLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getOverallLeaderboardMethod()

  public val coinsLeaderboardMethod:
      MethodDescriptor<Leaderboard.CoinsLeaderboardRequest, Leaderboard.CoinsLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getCoinsLeaderboardMethod()

  public val teamsLeaderboardMethod:
      MethodDescriptor<Leaderboard.TeamsLeaderboardRequest, Leaderboard.TeamsLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getTeamsLeaderboardMethod()

  public val kdaLeaderboardMethod:
      MethodDescriptor<Leaderboard.KdaLeaderboardRequest, Leaderboard.KdaLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getKdaLeaderboardMethod()

  public val deathsLeaderboardMethod:
      MethodDescriptor<Leaderboard.DeathsLeaderboardRequest, Leaderboard.DeathsLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getDeathsLeaderboardMethod()

  public val killsLeaderboardMethod:
      MethodDescriptor<Leaderboard.KillsLeaderboardRequest, Leaderboard.KillsLeaderboardResponse>
    @JvmStatic
    get() = BridgeGrpc.getKillsLeaderboardMethod()

  public val sendMessageMethod:
      MethodDescriptor<Messaging.SendMessageRequest, Messaging.SendMessageResponse>
    @JvmStatic
    get() = BridgeGrpc.getSendMessageMethod()

  public val getFriendsMethod:
      MethodDescriptor<Friends.GetFriendsRequest, Friends.GetFriendsResponse>
    @JvmStatic
    get() = BridgeGrpc.getGetFriendsMethod()

  public val sendFriendRequestMethod:
      MethodDescriptor<Friends.SendFriendRequestRequest, Friends.SendFriendRequestResponse>
    @JvmStatic
    get() = BridgeGrpc.getSendFriendRequestMethod()

  public val acceptFriendRequestMethod:
      MethodDescriptor<Friends.AcceptFriendRequestRequest, Friends.AcceptFriendRequestResponse>
    @JvmStatic
    get() = BridgeGrpc.getAcceptFriendRequestMethod()

  public val rejectFriendRequestMethod:
      MethodDescriptor<Friends.RejectFriendRequestRequest, Friends.RejectFriendRequestResponse>
    @JvmStatic
    get() = BridgeGrpc.getRejectFriendRequestMethod()

  public val getFriendRequestsMethod:
      MethodDescriptor<Friends.GetFriendRequestsRequest, Friends.GetFriendRequestsResponse>
    @JvmStatic
    get() = BridgeGrpc.getGetFriendRequestsMethod()

  public val removeFriendMethod:
      MethodDescriptor<Friends.RemoveFriendRequest, Friends.RemoveFriendResponse>
    @JvmStatic
    get() = BridgeGrpc.getRemoveFriendMethod()

  public val createTeamMethod: MethodDescriptor<Teams.CreateTeamRequest, Teams.CreateTeamResponse>
    @JvmStatic
    get() = BridgeGrpc.getCreateTeamMethod()

  public val leaveTeamMethod: MethodDescriptor<Teams.LeaveTeamRequest, Teams.LeaveTeamResponse>
    @JvmStatic
    get() = BridgeGrpc.getLeaveTeamMethod()

  public val joinTeamMethod: MethodDescriptor<Teams.JoinTeamRequest, Teams.JoinTeamResponse>
    @JvmStatic
    get() = BridgeGrpc.getJoinTeamMethod()

  public val sendTeamInviteMethod:
      MethodDescriptor<Teams.SendTeamInviteRequest, Teams.SendTeamInviteResponse>
    @JvmStatic
    get() = BridgeGrpc.getSendTeamInviteMethod()

  public val acceptTeamInviteMethod:
      MethodDescriptor<Teams.AcceptTeamInviteRequest, Teams.AcceptTeamInviteResponse>
    @JvmStatic
    get() = BridgeGrpc.getAcceptTeamInviteMethod()

  public val rejectTeamInviteMethod:
      MethodDescriptor<Teams.RejectTeamInviteRequest, Teams.RejectTeamInviteResponse>
    @JvmStatic
    get() = BridgeGrpc.getRejectTeamInviteMethod()

  public val getTeamMembersMethod:
      MethodDescriptor<Teams.GetTeamMembersRequest, Teams.GetTeamMembersResponse>
    @JvmStatic
    get() = BridgeGrpc.getGetTeamMembersMethod()

  public val removeTeamMemberMethod:
      MethodDescriptor<Friends.RemoveTeamMemberRequest, Friends.RemoveTeamMemberResponse>
    @JvmStatic
    get() = BridgeGrpc.getRemoveTeamMemberMethod()

  public val promoteTeamMemberMethod:
      MethodDescriptor<Teams.PromoteTeamMemberRequest, Teams.PromoteTeamMemberResponse>
    @JvmStatic
    get() = BridgeGrpc.getPromoteTeamMemberMethod()

  public val buyItemMethod: MethodDescriptor<Shop.BuyItemRequest, Shop.BuyItemResponse>
    @JvmStatic
    get() = BridgeGrpc.getBuyItemMethod()

  public val sellItemMethod: MethodDescriptor<Shop.SellItemRequest, Shop.SellItemResponse>
    @JvmStatic
    get() = BridgeGrpc.getSellItemMethod()

  public val getItemsMethod: MethodDescriptor<Shop.GetItemsRequest, Shop.GetItemsResponse>
    @JvmStatic
    get() = BridgeGrpc.getGetItemsMethod()

  public val playerDeathMethod:
      MethodDescriptor<Player.PlayerDeathRequest, Player.PlayerDeathResponse>
    @JvmStatic
    get() = BridgeGrpc.getPlayerDeathMethod()

  public val playerKillMethod: MethodDescriptor<Player.PlayerKillRequest, Player.PlayerKillResponse>
    @JvmStatic
    get() = BridgeGrpc.getPlayerKillMethod()

  public val proxyStartupMethod:
      MethodDescriptor<Server.ProxyStartupRequest, Server.ProxyStartupResponse>
    @JvmStatic
    get() = BridgeGrpc.getProxyStartupMethod()

  public val proxyShutdownMethod:
      MethodDescriptor<Server.ProxyShutdownRequest, Server.ProxyShutdownResponse>
    @JvmStatic
    get() = BridgeGrpc.getProxyShutdownMethod()

  public val survivalStartupMethod:
      MethodDescriptor<Server.SurvivalStartupRequest, Server.SurvivalStartupResponse>
    @JvmStatic
    get() = BridgeGrpc.getSurvivalStartupMethod()

  public val survivalShutdownMethod:
      MethodDescriptor<Server.SurvivalShutdownRequest, Server.SurvivalShutdownResponse>
    @JvmStatic
    get() = BridgeGrpc.getSurvivalShutdownMethod()

  public val lobbyStartupMethod:
      MethodDescriptor<Server.LobbyStartupRequest, Server.LobbyStartupResponse>
    @JvmStatic
    get() = BridgeGrpc.getLobbyStartupMethod()

  public val lobbyShutdownMethod:
      MethodDescriptor<Server.LobbyShutdownRequest, Server.LobbyShutdownResponse>
    @JvmStatic
    get() = BridgeGrpc.getLobbyShutdownMethod()

  public val serverSendMessageMethod:
      MethodDescriptor<Server.ServerSendMessageRequest, Server.ServerSendMessageResponse>
    @JvmStatic
    get() = BridgeGrpc.getServerSendMessageMethod()

  public val reportPlayerMethod:
      MethodDescriptor<Security.ReportPlayerRequest, Security.ReportPlayerResponse>
    @JvmStatic
    get() = BridgeGrpc.getReportPlayerMethod()

  /**
   * A stub for issuing RPCs to a(n) bridge.Bridge service as suspending coroutines.
   */
  @StubFor(BridgeGrpc::class)
  public class BridgeCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<BridgeCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): BridgeCoroutineStub =
        BridgeCoroutineStub(channel, callOptions)

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun playerJoin(request: Session.PlayerJoinRequest, headers: Metadata =
        Metadata()): Session.PlayerJoinResponse = unaryRpc(
      channel,
      BridgeGrpc.getPlayerJoinMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun playerLeave(request: Session.PlayerLeaveRequest, headers: Metadata =
        Metadata()): Session.PlayerLeaveResponse = unaryRpc(
      channel,
      BridgeGrpc.getPlayerLeaveMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun getBalance(request: Balance.GetBalanceRequest, headers: Metadata =
        Metadata()): Balance.GetBalanceResponse = unaryRpc(
      channel,
      BridgeGrpc.getGetBalanceMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun transferBalance(request: Balance.TransferBalanceRequest, headers: Metadata =
        Metadata()): Balance.TransferBalanceResponse = unaryRpc(
      channel,
      BridgeGrpc.getTransferBalanceMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun overallLeaderboard(request: Leaderboard.OverallLeaderboardRequest,
        headers: Metadata = Metadata()): Leaderboard.OverallLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getOverallLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun coinsLeaderboard(request: Leaderboard.CoinsLeaderboardRequest,
        headers: Metadata = Metadata()): Leaderboard.CoinsLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getCoinsLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun teamsLeaderboard(request: Leaderboard.TeamsLeaderboardRequest,
        headers: Metadata = Metadata()): Leaderboard.TeamsLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getTeamsLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun kdaLeaderboard(request: Leaderboard.KdaLeaderboardRequest, headers: Metadata
        = Metadata()): Leaderboard.KdaLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getKdaLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun deathsLeaderboard(request: Leaderboard.DeathsLeaderboardRequest,
        headers: Metadata = Metadata()): Leaderboard.DeathsLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getDeathsLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun killsLeaderboard(request: Leaderboard.KillsLeaderboardRequest,
        headers: Metadata = Metadata()): Leaderboard.KillsLeaderboardResponse = unaryRpc(
      channel,
      BridgeGrpc.getKillsLeaderboardMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun sendMessage(request: Messaging.SendMessageRequest, headers: Metadata =
        Metadata()): Messaging.SendMessageResponse = unaryRpc(
      channel,
      BridgeGrpc.getSendMessageMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun getFriends(request: Friends.GetFriendsRequest, headers: Metadata =
        Metadata()): Friends.GetFriendsResponse = unaryRpc(
      channel,
      BridgeGrpc.getGetFriendsMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun sendFriendRequest(request: Friends.SendFriendRequestRequest,
        headers: Metadata = Metadata()): Friends.SendFriendRequestResponse = unaryRpc(
      channel,
      BridgeGrpc.getSendFriendRequestMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun acceptFriendRequest(request: Friends.AcceptFriendRequestRequest,
        headers: Metadata = Metadata()): Friends.AcceptFriendRequestResponse = unaryRpc(
      channel,
      BridgeGrpc.getAcceptFriendRequestMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun rejectFriendRequest(request: Friends.RejectFriendRequestRequest,
        headers: Metadata = Metadata()): Friends.RejectFriendRequestResponse = unaryRpc(
      channel,
      BridgeGrpc.getRejectFriendRequestMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun getFriendRequests(request: Friends.GetFriendRequestsRequest,
        headers: Metadata = Metadata()): Friends.GetFriendRequestsResponse = unaryRpc(
      channel,
      BridgeGrpc.getGetFriendRequestsMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun removeFriend(request: Friends.RemoveFriendRequest, headers: Metadata =
        Metadata()): Friends.RemoveFriendResponse = unaryRpc(
      channel,
      BridgeGrpc.getRemoveFriendMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun createTeam(request: Teams.CreateTeamRequest, headers: Metadata = Metadata()):
        Teams.CreateTeamResponse = unaryRpc(
      channel,
      BridgeGrpc.getCreateTeamMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun leaveTeam(request: Teams.LeaveTeamRequest, headers: Metadata = Metadata()):
        Teams.LeaveTeamResponse = unaryRpc(
      channel,
      BridgeGrpc.getLeaveTeamMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun joinTeam(request: Teams.JoinTeamRequest, headers: Metadata = Metadata()):
        Teams.JoinTeamResponse = unaryRpc(
      channel,
      BridgeGrpc.getJoinTeamMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun sendTeamInvite(request: Teams.SendTeamInviteRequest, headers: Metadata =
        Metadata()): Teams.SendTeamInviteResponse = unaryRpc(
      channel,
      BridgeGrpc.getSendTeamInviteMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun acceptTeamInvite(request: Teams.AcceptTeamInviteRequest, headers: Metadata =
        Metadata()): Teams.AcceptTeamInviteResponse = unaryRpc(
      channel,
      BridgeGrpc.getAcceptTeamInviteMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun rejectTeamInvite(request: Teams.RejectTeamInviteRequest, headers: Metadata =
        Metadata()): Teams.RejectTeamInviteResponse = unaryRpc(
      channel,
      BridgeGrpc.getRejectTeamInviteMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun getTeamMembers(request: Teams.GetTeamMembersRequest, headers: Metadata =
        Metadata()): Teams.GetTeamMembersResponse = unaryRpc(
      channel,
      BridgeGrpc.getGetTeamMembersMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun removeTeamMember(request: Friends.RemoveTeamMemberRequest, headers: Metadata
        = Metadata()): Friends.RemoveTeamMemberResponse = unaryRpc(
      channel,
      BridgeGrpc.getRemoveTeamMemberMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun promoteTeamMember(request: Teams.PromoteTeamMemberRequest, headers: Metadata
        = Metadata()): Teams.PromoteTeamMemberResponse = unaryRpc(
      channel,
      BridgeGrpc.getPromoteTeamMemberMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun buyItem(request: Shop.BuyItemRequest, headers: Metadata = Metadata()):
        Shop.BuyItemResponse = unaryRpc(
      channel,
      BridgeGrpc.getBuyItemMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun sellItem(request: Shop.SellItemRequest, headers: Metadata = Metadata()):
        Shop.SellItemResponse = unaryRpc(
      channel,
      BridgeGrpc.getSellItemMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun getItems(request: Shop.GetItemsRequest, headers: Metadata = Metadata()):
        Shop.GetItemsResponse = unaryRpc(
      channel,
      BridgeGrpc.getGetItemsMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun playerDeath(request: Player.PlayerDeathRequest, headers: Metadata =
        Metadata()): Player.PlayerDeathResponse = unaryRpc(
      channel,
      BridgeGrpc.getPlayerDeathMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun playerKill(request: Player.PlayerKillRequest, headers: Metadata =
        Metadata()): Player.PlayerKillResponse = unaryRpc(
      channel,
      BridgeGrpc.getPlayerKillMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun proxyStartup(request: Server.ProxyStartupRequest, headers: Metadata =
        Metadata()): Server.ProxyStartupResponse = unaryRpc(
      channel,
      BridgeGrpc.getProxyStartupMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun proxyShutdown(request: Server.ProxyShutdownRequest, headers: Metadata =
        Metadata()): Server.ProxyShutdownResponse = unaryRpc(
      channel,
      BridgeGrpc.getProxyShutdownMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun survivalStartup(request: Server.SurvivalStartupRequest, headers: Metadata =
        Metadata()): Server.SurvivalStartupResponse = unaryRpc(
      channel,
      BridgeGrpc.getSurvivalStartupMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun survivalShutdown(request: Server.SurvivalShutdownRequest, headers: Metadata =
        Metadata()): Server.SurvivalShutdownResponse = unaryRpc(
      channel,
      BridgeGrpc.getSurvivalShutdownMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun lobbyStartup(request: Server.LobbyStartupRequest, headers: Metadata =
        Metadata()): Server.LobbyStartupResponse = unaryRpc(
      channel,
      BridgeGrpc.getLobbyStartupMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun lobbyShutdown(request: Server.LobbyShutdownRequest, headers: Metadata =
        Metadata()): Server.LobbyShutdownResponse = unaryRpc(
      channel,
      BridgeGrpc.getLobbyShutdownMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Returns a [Flow] that, when collected, executes this RPC and emits responses from the
     * server as they arrive.  That flow finishes normally if the server closes its response with
     * [`Status.OK`][io.grpc.Status], and fails by throwing a [StatusException] otherwise.  If
     * collecting the flow downstream fails exceptionally (including via cancellation), the RPC
     * is cancelled with that exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return A flow that, when collected, emits the responses from the server.
     */
    public fun serverSendMessage(request: Server.ServerSendMessageRequest, headers: Metadata =
        Metadata()): Flow<Server.ServerSendMessageResponse> = serverStreamingRpc(
      channel,
      BridgeGrpc.getServerSendMessageMethod(),
      request,
      callOptions,
      headers
    )

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a
     * corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun reportPlayer(request: Security.ReportPlayerRequest, headers: Metadata =
        Metadata()): Security.ReportPlayerResponse = unaryRpc(
      channel,
      BridgeGrpc.getReportPlayerMethod(),
      request,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the bridge.Bridge service based on Kotlin coroutines.
   */
  public abstract class BridgeCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns the response to an RPC for bridge.Bridge.PlayerJoin.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun playerJoin(request: Session.PlayerJoinRequest):
        Session.PlayerJoinResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.PlayerJoin is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.PlayerLeave.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun playerLeave(request: Session.PlayerLeaveRequest):
        Session.PlayerLeaveResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.PlayerLeave is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.GetBalance.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun getBalance(request: Balance.GetBalanceRequest):
        Balance.GetBalanceResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.GetBalance is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.TransferBalance.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun transferBalance(request: Balance.TransferBalanceRequest):
        Balance.TransferBalanceResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.TransferBalance is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.OverallLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun overallLeaderboard(request: Leaderboard.OverallLeaderboardRequest):
        Leaderboard.OverallLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.OverallLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.CoinsLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun coinsLeaderboard(request: Leaderboard.CoinsLeaderboardRequest):
        Leaderboard.CoinsLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.CoinsLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.TeamsLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun teamsLeaderboard(request: Leaderboard.TeamsLeaderboardRequest):
        Leaderboard.TeamsLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.TeamsLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.KdaLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun kdaLeaderboard(request: Leaderboard.KdaLeaderboardRequest):
        Leaderboard.KdaLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.KdaLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.DeathsLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun deathsLeaderboard(request: Leaderboard.DeathsLeaderboardRequest):
        Leaderboard.DeathsLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.DeathsLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.KillsLeaderboard.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun killsLeaderboard(request: Leaderboard.KillsLeaderboardRequest):
        Leaderboard.KillsLeaderboardResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.KillsLeaderboard is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SendMessage.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun sendMessage(request: Messaging.SendMessageRequest):
        Messaging.SendMessageResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SendMessage is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.GetFriends.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun getFriends(request: Friends.GetFriendsRequest):
        Friends.GetFriendsResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.GetFriends is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SendFriendRequest.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun sendFriendRequest(request: Friends.SendFriendRequestRequest):
        Friends.SendFriendRequestResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SendFriendRequest is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.AcceptFriendRequest.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun acceptFriendRequest(request: Friends.AcceptFriendRequestRequest):
        Friends.AcceptFriendRequestResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.AcceptFriendRequest is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.RejectFriendRequest.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun rejectFriendRequest(request: Friends.RejectFriendRequestRequest):
        Friends.RejectFriendRequestResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.RejectFriendRequest is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.GetFriendRequests.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun getFriendRequests(request: Friends.GetFriendRequestsRequest):
        Friends.GetFriendRequestsResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.GetFriendRequests is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.RemoveFriend.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun removeFriend(request: Friends.RemoveFriendRequest):
        Friends.RemoveFriendResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.RemoveFriend is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.CreateTeam.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun createTeam(request: Teams.CreateTeamRequest): Teams.CreateTeamResponse =
        throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.CreateTeam is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.LeaveTeam.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun leaveTeam(request: Teams.LeaveTeamRequest): Teams.LeaveTeamResponse =
        throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.LeaveTeam is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.JoinTeam.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun joinTeam(request: Teams.JoinTeamRequest): Teams.JoinTeamResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.JoinTeam is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SendTeamInvite.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun sendTeamInvite(request: Teams.SendTeamInviteRequest):
        Teams.SendTeamInviteResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SendTeamInvite is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.AcceptTeamInvite.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun acceptTeamInvite(request: Teams.AcceptTeamInviteRequest):
        Teams.AcceptTeamInviteResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.AcceptTeamInvite is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.RejectTeamInvite.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun rejectTeamInvite(request: Teams.RejectTeamInviteRequest):
        Teams.RejectTeamInviteResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.RejectTeamInvite is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.GetTeamMembers.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun getTeamMembers(request: Teams.GetTeamMembersRequest):
        Teams.GetTeamMembersResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.GetTeamMembers is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.RemoveTeamMember.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun removeTeamMember(request: Friends.RemoveTeamMemberRequest):
        Friends.RemoveTeamMemberResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.RemoveTeamMember is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.PromoteTeamMember.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun promoteTeamMember(request: Teams.PromoteTeamMemberRequest):
        Teams.PromoteTeamMemberResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.PromoteTeamMember is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.BuyItem.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun buyItem(request: Shop.BuyItemRequest): Shop.BuyItemResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.BuyItem is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SellItem.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun sellItem(request: Shop.SellItemRequest): Shop.SellItemResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SellItem is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.GetItems.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun getItems(request: Shop.GetItemsRequest): Shop.GetItemsResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.GetItems is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.PlayerDeath.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun playerDeath(request: Player.PlayerDeathRequest):
        Player.PlayerDeathResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.PlayerDeath is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.PlayerKill.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun playerKill(request: Player.PlayerKillRequest): Player.PlayerKillResponse
        = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.PlayerKill is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.ProxyStartup.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun proxyStartup(request: Server.ProxyStartupRequest):
        Server.ProxyStartupResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.ProxyStartup is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.ProxyShutdown.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun proxyShutdown(request: Server.ProxyShutdownRequest):
        Server.ProxyShutdownResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.ProxyShutdown is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SurvivalStartup.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun survivalStartup(request: Server.SurvivalStartupRequest):
        Server.SurvivalStartupResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SurvivalStartup is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.SurvivalShutdown.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun survivalShutdown(request: Server.SurvivalShutdownRequest):
        Server.SurvivalShutdownResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.SurvivalShutdown is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.LobbyStartup.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun lobbyStartup(request: Server.LobbyStartupRequest):
        Server.LobbyStartupResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.LobbyStartup is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.LobbyShutdown.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun lobbyShutdown(request: Server.LobbyShutdownRequest):
        Server.LobbyShutdownResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.LobbyShutdown is unimplemented"))

    /**
     * Returns a [Flow] of responses to an RPC for bridge.Bridge.ServerSendMessage.
     *
     * If creating or collecting the returned flow fails with a [StatusException], the RPC
     * will fail with the corresponding [io.grpc.Status].  If it fails with a
     * [java.util.concurrent.CancellationException], the RPC will fail with status
     * `Status.CANCELLED`.  If creating
     * or collecting the returned flow fails for any other reason, the RPC will fail with
     * `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open fun serverSendMessage(request: Server.ServerSendMessageRequest):
        Flow<Server.ServerSendMessageResponse> = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.ServerSendMessage is unimplemented"))

    /**
     * Returns the response to an RPC for bridge.Bridge.ReportPlayer.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
     * the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun reportPlayer(request: Security.ReportPlayerRequest):
        Security.ReportPlayerResponse = throw
        StatusException(UNIMPLEMENTED.withDescription("Method bridge.Bridge.ReportPlayer is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(getServiceDescriptor())
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getPlayerJoinMethod(),
      implementation = ::playerJoin
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getPlayerLeaveMethod(),
      implementation = ::playerLeave
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getGetBalanceMethod(),
      implementation = ::getBalance
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getTransferBalanceMethod(),
      implementation = ::transferBalance
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getOverallLeaderboardMethod(),
      implementation = ::overallLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getCoinsLeaderboardMethod(),
      implementation = ::coinsLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getTeamsLeaderboardMethod(),
      implementation = ::teamsLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getKdaLeaderboardMethod(),
      implementation = ::kdaLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getDeathsLeaderboardMethod(),
      implementation = ::deathsLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getKillsLeaderboardMethod(),
      implementation = ::killsLeaderboard
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSendMessageMethod(),
      implementation = ::sendMessage
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getGetFriendsMethod(),
      implementation = ::getFriends
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSendFriendRequestMethod(),
      implementation = ::sendFriendRequest
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getAcceptFriendRequestMethod(),
      implementation = ::acceptFriendRequest
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getRejectFriendRequestMethod(),
      implementation = ::rejectFriendRequest
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getGetFriendRequestsMethod(),
      implementation = ::getFriendRequests
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getRemoveFriendMethod(),
      implementation = ::removeFriend
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getCreateTeamMethod(),
      implementation = ::createTeam
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getLeaveTeamMethod(),
      implementation = ::leaveTeam
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getJoinTeamMethod(),
      implementation = ::joinTeam
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSendTeamInviteMethod(),
      implementation = ::sendTeamInvite
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getAcceptTeamInviteMethod(),
      implementation = ::acceptTeamInvite
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getRejectTeamInviteMethod(),
      implementation = ::rejectTeamInvite
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getGetTeamMembersMethod(),
      implementation = ::getTeamMembers
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getRemoveTeamMemberMethod(),
      implementation = ::removeTeamMember
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getPromoteTeamMemberMethod(),
      implementation = ::promoteTeamMember
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getBuyItemMethod(),
      implementation = ::buyItem
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSellItemMethod(),
      implementation = ::sellItem
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getGetItemsMethod(),
      implementation = ::getItems
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getPlayerDeathMethod(),
      implementation = ::playerDeath
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getPlayerKillMethod(),
      implementation = ::playerKill
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getProxyStartupMethod(),
      implementation = ::proxyStartup
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getProxyShutdownMethod(),
      implementation = ::proxyShutdown
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSurvivalStartupMethod(),
      implementation = ::survivalStartup
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getSurvivalShutdownMethod(),
      implementation = ::survivalShutdown
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getLobbyStartupMethod(),
      implementation = ::lobbyStartup
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getLobbyShutdownMethod(),
      implementation = ::lobbyShutdown
    ))
      .addMethod(serverStreamingServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getServerSendMessageMethod(),
      implementation = ::serverSendMessage
    ))
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = BridgeGrpc.getReportPlayerMethod(),
      implementation = ::reportPlayer
    )).build()
  }
}
