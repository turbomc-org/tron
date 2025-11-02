package com.tron.bridge;

/**
 * Holder for Kotlin coroutine-based client and server APIs for bridge.Bridge.
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0096\u0005\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0010\u000e\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0004\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0004\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0004\u00df\u0001\u00e0\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002R\u000e\u0010\u0003\u001a\u00020\u0004X\u0086T\u00a2\u0006\u0002\n\u0000R\u001d\u0010\u0005\u001a\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00068G\u00a2\u0006\u0006\u001a\u0004\b\t\u0010\nR\u001d\u0010\u000b\u001a\u000e\u0012\u0004\u0012\u00020\f\u0012\u0004\u0012\u00020\r0\u00068G\u00a2\u0006\u0006\u001a\u0004\b\u000e\u0010\nR\u001d\u0010\u000f\u001a\u000e\u0012\u0004\u0012\u00020\u0010\u0012\u0004\u0012\u00020\u00110\u00068G\u00a2\u0006\u0006\u001a\u0004\b\u0012\u0010\nR\u001d\u0010\u0013\u001a\u000e\u0012\u0004\u0012\u00020\u0014\u0012\u0004\u0012\u00020\u00150\u00068G\u00a2\u0006\u0006\u001a\u0004\b\u0016\u0010\nR\u001d\u0010\u0017\u001a\u000e\u0012\u0004\u0012\u00020\u0018\u0012\u0004\u0012\u00020\u00190\u00068G\u00a2\u0006\u0006\u001a\u0004\b\u001a\u0010\nR\u001d\u0010\u001b\u001a\u000e\u0012\u0004\u0012\u00020\u001c\u0012\u0004\u0012\u00020\u001d0\u00068G\u00a2\u0006\u0006\u001a\u0004\b\u001e\u0010\nR\u001d\u0010\u001f\u001a\u000e\u0012\u0004\u0012\u00020 \u0012\u0004\u0012\u00020!0\u00068G\u00a2\u0006\u0006\u001a\u0004\b\"\u0010\nR\u001d\u0010#\u001a\u000e\u0012\u0004\u0012\u00020$\u0012\u0004\u0012\u00020%0\u00068G\u00a2\u0006\u0006\u001a\u0004\b&\u0010\nR\u001d\u0010\'\u001a\u000e\u0012\u0004\u0012\u00020(\u0012\u0004\u0012\u00020)0\u00068G\u00a2\u0006\u0006\u001a\u0004\b*\u0010\nR\u001d\u0010+\u001a\u000e\u0012\u0004\u0012\u00020,\u0012\u0004\u0012\u00020-0\u00068G\u00a2\u0006\u0006\u001a\u0004\b.\u0010\nR\u001d\u0010/\u001a\u000e\u0012\u0004\u0012\u000200\u0012\u0004\u0012\u0002010\u00068G\u00a2\u0006\u0006\u001a\u0004\b2\u0010\nR\u001d\u00103\u001a\u000e\u0012\u0004\u0012\u000204\u0012\u0004\u0012\u0002050\u00068G\u00a2\u0006\u0006\u001a\u0004\b6\u0010\nR\u001d\u00107\u001a\u000e\u0012\u0004\u0012\u000208\u0012\u0004\u0012\u0002090\u00068G\u00a2\u0006\u0006\u001a\u0004\b:\u0010\nR\u001d\u0010;\u001a\u000e\u0012\u0004\u0012\u00020<\u0012\u0004\u0012\u00020=0\u00068G\u00a2\u0006\u0006\u001a\u0004\b>\u0010\nR\u001d\u0010?\u001a\u000e\u0012\u0004\u0012\u00020@\u0012\u0004\u0012\u00020A0\u00068G\u00a2\u0006\u0006\u001a\u0004\bB\u0010\nR\u001d\u0010C\u001a\u000e\u0012\u0004\u0012\u00020D\u0012\u0004\u0012\u00020E0\u00068G\u00a2\u0006\u0006\u001a\u0004\bF\u0010\nR\u001d\u0010G\u001a\u000e\u0012\u0004\u0012\u00020H\u0012\u0004\u0012\u00020I0\u00068G\u00a2\u0006\u0006\u001a\u0004\bJ\u0010\nR\u001d\u0010K\u001a\u000e\u0012\u0004\u0012\u00020L\u0012\u0004\u0012\u00020M0\u00068G\u00a2\u0006\u0006\u001a\u0004\bN\u0010\nR\u001d\u0010O\u001a\u000e\u0012\u0004\u0012\u00020P\u0012\u0004\u0012\u00020Q0\u00068G\u00a2\u0006\u0006\u001a\u0004\bR\u0010\nR\u001d\u0010S\u001a\u000e\u0012\u0004\u0012\u00020T\u0012\u0004\u0012\u00020U0\u00068G\u00a2\u0006\u0006\u001a\u0004\bV\u0010\nR\u001d\u0010W\u001a\u000e\u0012\u0004\u0012\u00020X\u0012\u0004\u0012\u00020Y0\u00068G\u00a2\u0006\u0006\u001a\u0004\bZ\u0010\nR\u001d\u0010[\u001a\u000e\u0012\u0004\u0012\u00020\\\u0012\u0004\u0012\u00020]0\u00068G\u00a2\u0006\u0006\u001a\u0004\b^\u0010\nR\u001d\u0010_\u001a\u000e\u0012\u0004\u0012\u00020`\u0012\u0004\u0012\u00020a0\u00068G\u00a2\u0006\u0006\u001a\u0004\bb\u0010\nR\u001d\u0010c\u001a\u000e\u0012\u0004\u0012\u00020d\u0012\u0004\u0012\u00020e0\u00068G\u00a2\u0006\u0006\u001a\u0004\bf\u0010\nR\u001d\u0010g\u001a\u000e\u0012\u0004\u0012\u00020h\u0012\u0004\u0012\u00020i0\u00068G\u00a2\u0006\u0006\u001a\u0004\bj\u0010\nR\u001d\u0010k\u001a\u000e\u0012\u0004\u0012\u00020l\u0012\u0004\u0012\u00020m0\u00068G\u00a2\u0006\u0006\u001a\u0004\bn\u0010\nR\u001d\u0010o\u001a\u000e\u0012\u0004\u0012\u00020p\u0012\u0004\u0012\u00020q0\u00068G\u00a2\u0006\u0006\u001a\u0004\br\u0010\nR\u001d\u0010s\u001a\u000e\u0012\u0004\u0012\u00020t\u0012\u0004\u0012\u00020u0\u00068G\u00a2\u0006\u0006\u001a\u0004\bv\u0010\nR\u001d\u0010w\u001a\u000e\u0012\u0004\u0012\u00020x\u0012\u0004\u0012\u00020y0\u00068G\u00a2\u0006\u0006\u001a\u0004\bz\u0010\nR\u001d\u0010{\u001a\u000e\u0012\u0004\u0012\u00020|\u0012\u0004\u0012\u00020}0\u00068G\u00a2\u0006\u0006\u001a\u0004\b~\u0010\nR \u0010\u007f\u001a\u0010\u0012\u0005\u0012\u00030\u0080\u0001\u0012\u0005\u0012\u00030\u0081\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u0082\u0001\u0010\nR!\u0010\u0083\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u0084\u0001\u0012\u0005\u0012\u00030\u0085\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u0086\u0001\u0010\nR!\u0010\u0087\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u0088\u0001\u0012\u0005\u0012\u00030\u0089\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u008a\u0001\u0010\nR!\u0010\u008b\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u008c\u0001\u0012\u0005\u0012\u00030\u008d\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u008e\u0001\u0010\nR!\u0010\u008f\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u0090\u0001\u0012\u0005\u0012\u00030\u0091\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u0092\u0001\u0010\nR!\u0010\u0093\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u0094\u0001\u0012\u0005\u0012\u00030\u0095\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u0096\u0001\u0010\nR!\u0010\u0097\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u0098\u0001\u0012\u0005\u0012\u00030\u0099\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u009a\u0001\u0010\nR!\u0010\u009b\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u009c\u0001\u0012\u0005\u0012\u00030\u009d\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u009e\u0001\u0010\nR!\u0010\u009f\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00a0\u0001\u0012\u0005\u0012\u00030\u00a1\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00a2\u0001\u0010\nR!\u0010\u00a3\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00a4\u0001\u0012\u0005\u0012\u00030\u00a5\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00a6\u0001\u0010\nR!\u0010\u00a7\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00a8\u0001\u0012\u0005\u0012\u00030\u00a9\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00aa\u0001\u0010\nR!\u0010\u00ab\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00ac\u0001\u0012\u0005\u0012\u00030\u00ad\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00ae\u0001\u0010\nR!\u0010\u00af\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00b0\u0001\u0012\u0005\u0012\u00030\u00b1\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00b2\u0001\u0010\nR!\u0010\u00b3\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00b4\u0001\u0012\u0005\u0012\u00030\u00b5\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00b6\u0001\u0010\nR!\u0010\u00b7\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00b8\u0001\u0012\u0005\u0012\u00030\u00b9\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00ba\u0001\u0010\nR!\u0010\u00bb\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00bc\u0001\u0012\u0005\u0012\u00030\u00bd\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00be\u0001\u0010\nR!\u0010\u00bf\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00c0\u0001\u0012\u0005\u0012\u00030\u00c1\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00c2\u0001\u0010\nR!\u0010\u00c3\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00c4\u0001\u0012\u0005\u0012\u00030\u00c5\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00c6\u0001\u0010\nR!\u0010\u00c7\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00c4\u0001\u0012\u0005\u0012\u00030\u00c8\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00c9\u0001\u0010\nR\u001f\u0010\u00ca\u0001\u001a\u00030\u00cb\u00018FX\u0087\u0004\u00a2\u0006\u000f\u0012\u0005\b\u00cc\u0001\u0010\u0002\u001a\u0006\b\u00cd\u0001\u0010\u00ce\u0001R!\u0010\u00cf\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00d0\u0001\u0012\u0005\u0012\u00030\u00d1\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00d2\u0001\u0010\nR!\u0010\u00d3\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00d4\u0001\u0012\u0005\u0012\u00030\u00d5\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00d6\u0001\u0010\nR!\u0010\u00d7\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00d8\u0001\u0012\u0005\u0012\u00030\u00d9\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00da\u0001\u0010\nR!\u0010\u00db\u0001\u001a\u0010\u0012\u0005\u0012\u00030\u00dc\u0001\u0012\u0005\u0012\u00030\u00dd\u00010\u00068G\u00a2\u0006\u0007\u001a\u0005\b\u00de\u0001\u0010\n\u00a8\u0006\u00e1\u0001"}, d2 = {"Lcom/tron/bridge/BridgeGrpcKt;", "", "()V", "SERVICE_NAME", "", "acceptFriendRequestMethod", "Lio/grpc/MethodDescriptor;", "Lcom/tron/bridge/Friends$AcceptFriendRequestRequest;", "Lcom/tron/bridge/Friends$AcceptFriendRequestResponse;", "getAcceptFriendRequestMethod", "()Lio/grpc/MethodDescriptor;", "acceptTeamInviteMethod", "Lcom/tron/bridge/Teams$AcceptTeamInviteRequest;", "Lcom/tron/bridge/Teams$AcceptTeamInviteResponse;", "getAcceptTeamInviteMethod", "buyItemMethod", "Lcom/tron/bridge/Shop$BuyItemRequest;", "Lcom/tron/bridge/Shop$BuyItemResponse;", "getBuyItemMethod", "buyPrefixMethod", "Lcom/tron/bridge/Prefix$BuyPrefixRequest;", "Lcom/tron/bridge/Prefix$BuyPrefixResponse;", "getBuyPrefixMethod", "coinsLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardResponse;", "getCoinsLeaderboardMethod", "createPrefixMethod", "Lcom/tron/bridge/Prefix$CreatePrefixRequest;", "Lcom/tron/bridge/Prefix$CreatePrefixResponse;", "getCreatePrefixMethod", "createTeamMethod", "Lcom/tron/bridge/Teams$CreateTeamRequest;", "Lcom/tron/bridge/Teams$CreateTeamResponse;", "getCreateTeamMethod", "deathsLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardResponse;", "getDeathsLeaderboardMethod", "deletePrefixMethod", "Lcom/tron/bridge/Prefix$DeletePrefixRequest;", "Lcom/tron/bridge/Prefix$DeletePrefixResponse;", "getDeletePrefixMethod", "deleteTeamMethod", "Lcom/tron/bridge/Teams$DeleteTeamRequest;", "Lcom/tron/bridge/Teams$DeleteTeamResponse;", "getDeleteTeamMethod", "getAllPrefixMethod", "Lcom/tron/bridge/Prefix$GetAllPrefixRequest;", "Lcom/tron/bridge/Prefix$GetAllPrefixResponse;", "getGetAllPrefixMethod", "getBalanceMethod", "Lcom/tron/bridge/Balance$GetBalanceRequest;", "Lcom/tron/bridge/Balance$GetBalanceResponse;", "getGetBalanceMethod", "getCurrentPrefixMethod", "Lcom/tron/bridge/Prefix$GetCurrentPrefixRequest;", "Lcom/tron/bridge/Prefix$GetCurrentPrefixResponse;", "getGetCurrentPrefixMethod", "getFriendRequestsMethod", "Lcom/tron/bridge/Friends$GetFriendRequestsRequest;", "Lcom/tron/bridge/Friends$GetFriendRequestsResponse;", "getGetFriendRequestsMethod", "getFriendsMethod", "Lcom/tron/bridge/Friends$GetFriendsRequest;", "Lcom/tron/bridge/Friends$GetFriendsResponse;", "getGetFriendsMethod", "getItemsMethod", "Lcom/tron/bridge/Shop$GetItemsRequest;", "Lcom/tron/bridge/Shop$GetItemsResponse;", "getGetItemsMethod", "getOpenTeamsMethod", "Lcom/tron/bridge/Teams$GetOpenTeamsRequest;", "Lcom/tron/bridge/Teams$GetOpenTeamsResponse;", "getGetOpenTeamsMethod", "getOwnedPrefixMethod", "Lcom/tron/bridge/Prefix$GetOwnedPrefixRequest;", "Lcom/tron/bridge/Prefix$GetOwnedPrefixResponse;", "getGetOwnedPrefixMethod", "getTeamMembersMethod", "Lcom/tron/bridge/Teams$GetTeamMembersRequest;", "Lcom/tron/bridge/Teams$GetTeamMembersResponse;", "getGetTeamMembersMethod", "joinTeamMethod", "Lcom/tron/bridge/Teams$JoinTeamRequest;", "Lcom/tron/bridge/Teams$JoinTeamResponse;", "getJoinTeamMethod", "kdaLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardResponse;", "getKdaLeaderboardMethod", "killsLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardResponse;", "getKillsLeaderboardMethod", "leaveTeamMethod", "Lcom/tron/bridge/Teams$LeaveTeamRequest;", "Lcom/tron/bridge/Teams$LeaveTeamResponse;", "getLeaveTeamMethod", "listFriendRequestsMethod", "Lcom/tron/bridge/Friends$ListFriendRequestsRequest;", "Lcom/tron/bridge/Friends$ListFriendRequestsResponse;", "getListFriendRequestsMethod", "listFriendsMethod", "Lcom/tron/bridge/Friends$ListFriendsRequest;", "Lcom/tron/bridge/Friends$ListFriendsResponse;", "getListFriendsMethod", "lobbyShutdownMethod", "Lcom/tron/bridge/Server$LobbyShutdownRequest;", "Lcom/tron/bridge/Server$LobbyShutdownResponse;", "getLobbyShutdownMethod", "lobbyStartupMethod", "Lcom/tron/bridge/Server$LobbyStartupRequest;", "Lcom/tron/bridge/Server$LobbyStartupResponse;", "getLobbyStartupMethod", "messageMethod", "Lcom/tron/bridge/Server$MessageRequest;", "Lcom/tron/bridge/Server$MessageResponse;", "getMessageMethod", "overallLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardResponse;", "getOverallLeaderboardMethod", "playerBreakBlockMethod", "Lcom/tron/bridge/Player$PlayerBreakBlockRequest;", "Lcom/tron/bridge/Player$PlayerBreakBlockResponse;", "getPlayerBreakBlockMethod", "playerDeathMethod", "Lcom/tron/bridge/Player$PlayerDeathRequest;", "Lcom/tron/bridge/Player$PlayerDeathResponse;", "getPlayerDeathMethod", "playerJoinMethod", "Lcom/tron/bridge/Session$PlayerJoinRequest;", "Lcom/tron/bridge/Session$PlayerJoinResponse;", "getPlayerJoinMethod", "playerKillMethod", "Lcom/tron/bridge/Player$PlayerKillRequest;", "Lcom/tron/bridge/Player$PlayerKillResponse;", "getPlayerKillMethod", "playerLeaveMethod", "Lcom/tron/bridge/Session$PlayerLeaveRequest;", "Lcom/tron/bridge/Session$PlayerLeaveResponse;", "getPlayerLeaveMethod", "playerPlaceBlockMethod", "Lcom/tron/bridge/Player$PlayerPlaceBlockRequest;", "Lcom/tron/bridge/Player$PlayerPlaceBlockResponse;", "getPlayerPlaceBlockMethod", "promoteTeamMemberMethod", "Lcom/tron/bridge/Teams$PromoteTeamMemberRequest;", "Lcom/tron/bridge/Teams$PromoteTeamMemberResponse;", "getPromoteTeamMemberMethod", "proxyShutdownMethod", "Lcom/tron/bridge/Server$ProxyShutdownRequest;", "Lcom/tron/bridge/Server$ProxyShutdownResponse;", "getProxyShutdownMethod", "proxyStartupMethod", "Lcom/tron/bridge/Server$ProxyStartupRequest;", "Lcom/tron/bridge/Server$ProxyStartupResponse;", "getProxyStartupMethod", "rejectFriendRequestMethod", "Lcom/tron/bridge/Friends$RejectFriendRequestRequest;", "Lcom/tron/bridge/Friends$RejectFriendRequestResponse;", "getRejectFriendRequestMethod", "rejectTeamInviteMethod", "Lcom/tron/bridge/Teams$RejectTeamInviteRequest;", "Lcom/tron/bridge/Teams$RejectTeamInviteResponse;", "getRejectTeamInviteMethod", "removeFriendMethod", "Lcom/tron/bridge/Friends$RemoveFriendRequest;", "Lcom/tron/bridge/Friends$RemoveFriendResponse;", "getRemoveFriendMethod", "removeTeamMemberMethod", "Lcom/tron/bridge/Friends$RemoveTeamMemberRequest;", "Lcom/tron/bridge/Friends$RemoveTeamMemberResponse;", "getRemoveTeamMemberMethod", "reportPlayerMethod", "Lcom/tron/bridge/Security$ReportPlayerRequest;", "Lcom/tron/bridge/Security$ReportPlayerResponse;", "getReportPlayerMethod", "selectPrefixMethod", "Lcom/tron/bridge/Prefix$SelectPrefixRequest;", "Lcom/tron/bridge/Prefix$SelectPrefixResponse;", "getSelectPrefixMethod", "sellItemMethod", "Lcom/tron/bridge/Shop$SellItemRequest;", "Lcom/tron/bridge/Shop$SellItemResponse;", "getSellItemMethod", "sendFriendRequestMethod", "Lcom/tron/bridge/Friends$SendFriendRequestRequest;", "Lcom/tron/bridge/Friends$SendFriendRequestResponse;", "getSendFriendRequestMethod", "sendTeamInviteMethod", "Lcom/tron/bridge/Teams$SendTeamInviteRequest;", "Lcom/tron/bridge/Teams$SendTeamInviteResponse;", "getSendTeamInviteMethod", "serverSendMessageMethod", "Lcom/tron/bridge/Server$ServerSubscribeRequest;", "Lcom/tron/bridge/Server$ServerSendMessageResponse;", "getServerSendMessageMethod", "serverSendTitleMethod", "Lcom/tron/bridge/Server$ServerSendTitleResponse;", "getServerSendTitleMethod", "serviceDescriptor", "Lio/grpc/ServiceDescriptor;", "getServiceDescriptor$annotations", "getServiceDescriptor", "()Lio/grpc/ServiceDescriptor;", "survivalShutdownMethod", "Lcom/tron/bridge/Server$SurvivalShutdownRequest;", "Lcom/tron/bridge/Server$SurvivalShutdownResponse;", "getSurvivalShutdownMethod", "survivalStartupMethod", "Lcom/tron/bridge/Server$SurvivalStartupRequest;", "Lcom/tron/bridge/Server$SurvivalStartupResponse;", "getSurvivalStartupMethod", "teamsLeaderboardMethod", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardRequest;", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardResponse;", "getTeamsLeaderboardMethod", "transferBalanceMethod", "Lcom/tron/bridge/Balance$TransferBalanceRequest;", "Lcom/tron/bridge/Balance$TransferBalanceResponse;", "getTransferBalanceMethod", "BridgeCoroutineImplBase", "BridgeCoroutineStub", "proxy"})
public final class BridgeGrpcKt {
    @org.jetbrains.annotations.NotNull()
    public static final java.lang.String SERVICE_NAME = "bridge.Bridge";
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.BridgeGrpcKt INSTANCE = null;
    
    private BridgeGrpcKt() {
        super();
    }
    
    @kotlin.jvm.JvmStatic()
    @java.lang.Deprecated()
    public static void getServiceDescriptor$annotations() {
    }
    
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.ServiceDescriptor getServiceDescriptor() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerJoinRequest, com.tron.bridge.Session.PlayerJoinResponse> getPlayerJoinMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerLeaveRequest, com.tron.bridge.Session.PlayerLeaveResponse> getPlayerLeaveMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Balance.GetBalanceRequest, com.tron.bridge.Balance.GetBalanceResponse> getGetBalanceMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Balance.TransferBalanceRequest, com.tron.bridge.Balance.TransferBalanceResponse> getTransferBalanceMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.OverallLeaderboardRequest, com.tron.bridge.Leaderboard.OverallLeaderboardResponse> getOverallLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.CoinsLeaderboardRequest, com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> getCoinsLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.TeamsLeaderboardRequest, com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> getTeamsLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KdaLeaderboardRequest, com.tron.bridge.Leaderboard.KdaLeaderboardResponse> getKdaLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.DeathsLeaderboardRequest, com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> getDeathsLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KillsLeaderboardRequest, com.tron.bridge.Leaderboard.KillsLeaderboardResponse> getKillsLeaderboardMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendsRequest, com.tron.bridge.Friends.GetFriendsResponse> getGetFriendsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendsRequest, com.tron.bridge.Friends.ListFriendsResponse> getListFriendsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.SendFriendRequestRequest, com.tron.bridge.Friends.SendFriendRequestResponse> getSendFriendRequestMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.AcceptFriendRequestRequest, com.tron.bridge.Friends.AcceptFriendRequestResponse> getAcceptFriendRequestMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.RejectFriendRequestRequest, com.tron.bridge.Friends.RejectFriendRequestResponse> getRejectFriendRequestMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendRequestsRequest, com.tron.bridge.Friends.GetFriendRequestsResponse> getGetFriendRequestsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendRequestsRequest, com.tron.bridge.Friends.ListFriendRequestsResponse> getListFriendRequestsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveFriendRequest, com.tron.bridge.Friends.RemoveFriendResponse> getRemoveFriendMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.CreateTeamRequest, com.tron.bridge.Teams.CreateTeamResponse> getCreateTeamMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.DeleteTeamRequest, com.tron.bridge.Teams.DeleteTeamResponse> getDeleteTeamMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.LeaveTeamRequest, com.tron.bridge.Teams.LeaveTeamResponse> getLeaveTeamMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.JoinTeamRequest, com.tron.bridge.Teams.JoinTeamResponse> getJoinTeamMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.SendTeamInviteRequest, com.tron.bridge.Teams.SendTeamInviteResponse> getSendTeamInviteMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.AcceptTeamInviteRequest, com.tron.bridge.Teams.AcceptTeamInviteResponse> getAcceptTeamInviteMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.RejectTeamInviteRequest, com.tron.bridge.Teams.RejectTeamInviteResponse> getRejectTeamInviteMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetTeamMembersRequest, com.tron.bridge.Teams.GetTeamMembersResponse> getGetTeamMembersMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveTeamMemberRequest, com.tron.bridge.Friends.RemoveTeamMemberResponse> getRemoveTeamMemberMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.PromoteTeamMemberRequest, com.tron.bridge.Teams.PromoteTeamMemberResponse> getPromoteTeamMemberMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetOpenTeamsRequest, com.tron.bridge.Teams.GetOpenTeamsResponse> getGetOpenTeamsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Shop.BuyItemRequest, com.tron.bridge.Shop.BuyItemResponse> getBuyItemMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Shop.SellItemRequest, com.tron.bridge.Shop.SellItemResponse> getSellItemMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Shop.GetItemsRequest, com.tron.bridge.Shop.GetItemsResponse> getGetItemsMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.BuyPrefixRequest, com.tron.bridge.Prefix.BuyPrefixResponse> getBuyPrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.SelectPrefixRequest, com.tron.bridge.Prefix.SelectPrefixResponse> getSelectPrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetAllPrefixRequest, com.tron.bridge.Prefix.GetAllPrefixResponse> getGetAllPrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetOwnedPrefixRequest, com.tron.bridge.Prefix.GetOwnedPrefixResponse> getGetOwnedPrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetCurrentPrefixRequest, com.tron.bridge.Prefix.GetCurrentPrefixResponse> getGetCurrentPrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.CreatePrefixRequest, com.tron.bridge.Prefix.CreatePrefixResponse> getCreatePrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Prefix.DeletePrefixRequest, com.tron.bridge.Prefix.DeletePrefixResponse> getDeletePrefixMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerDeathRequest, com.tron.bridge.Player.PlayerDeathResponse> getPlayerDeathMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerKillRequest, com.tron.bridge.Player.PlayerKillResponse> getPlayerKillMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerPlaceBlockRequest, com.tron.bridge.Player.PlayerPlaceBlockResponse> getPlayerPlaceBlockMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerBreakBlockRequest, com.tron.bridge.Player.PlayerBreakBlockResponse> getPlayerBreakBlockMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyStartupRequest, com.tron.bridge.Server.ProxyStartupResponse> getProxyStartupMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyShutdownRequest, com.tron.bridge.Server.ProxyShutdownResponse> getProxyShutdownMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalStartupRequest, com.tron.bridge.Server.SurvivalStartupResponse> getSurvivalStartupMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalShutdownRequest, com.tron.bridge.Server.SurvivalShutdownResponse> getSurvivalShutdownMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyStartupRequest, com.tron.bridge.Server.LobbyStartupResponse> getLobbyStartupMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyShutdownRequest, com.tron.bridge.Server.LobbyShutdownResponse> getLobbyShutdownMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendMessageResponse> getServerSendMessageMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendTitleResponse> getServerSendTitleMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Server.MessageRequest, com.tron.bridge.Server.MessageResponse> getMessageMethod() {
        return null;
    }
    
    @kotlin.jvm.JvmStatic()
    @org.jetbrains.annotations.NotNull()
    public static final io.grpc.MethodDescriptor<com.tron.bridge.Security.ReportPlayerRequest, com.tron.bridge.Security.ReportPlayerResponse> getReportPlayerMethod() {
        return null;
    }
    
    /**
     * Skeletal implementation of the bridge.Bridge service based on Kotlin coroutines.
     */
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0092\u0005\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\b&\u0018\u00002\u00020\u0001B\u000f\u0012\b\b\u0002\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\u0016\u0010\u0005\u001a\u00020\u00062\u0006\u0010\u0007\u001a\u00020\bH\u0096@\u00a2\u0006\u0002\u0010\tJ\u0016\u0010\n\u001a\u00020\u000b2\u0006\u0010\u0007\u001a\u00020\fH\u0096@\u00a2\u0006\u0002\u0010\rJ\u0006\u0010\u000e\u001a\u00020\u000fJ\u0016\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u0007\u001a\u00020\u0012H\u0096@\u00a2\u0006\u0002\u0010\u0013J\u0016\u0010\u0014\u001a\u00020\u00152\u0006\u0010\u0007\u001a\u00020\u0016H\u0096@\u00a2\u0006\u0002\u0010\u0017J\u0016\u0010\u0018\u001a\u00020\u00192\u0006\u0010\u0007\u001a\u00020\u001aH\u0096@\u00a2\u0006\u0002\u0010\u001bJ\u0016\u0010\u001c\u001a\u00020\u001d2\u0006\u0010\u0007\u001a\u00020\u001eH\u0096@\u00a2\u0006\u0002\u0010\u001fJ\u0016\u0010 \u001a\u00020!2\u0006\u0010\u0007\u001a\u00020\"H\u0096@\u00a2\u0006\u0002\u0010#J\u0016\u0010$\u001a\u00020%2\u0006\u0010\u0007\u001a\u00020&H\u0096@\u00a2\u0006\u0002\u0010\'J\u0016\u0010(\u001a\u00020)2\u0006\u0010\u0007\u001a\u00020*H\u0096@\u00a2\u0006\u0002\u0010+J\u0016\u0010,\u001a\u00020-2\u0006\u0010\u0007\u001a\u00020.H\u0096@\u00a2\u0006\u0002\u0010/J\u0016\u00100\u001a\u0002012\u0006\u0010\u0007\u001a\u000202H\u0096@\u00a2\u0006\u0002\u00103J\u0016\u00104\u001a\u0002052\u0006\u0010\u0007\u001a\u000206H\u0096@\u00a2\u0006\u0002\u00107J\u0016\u00108\u001a\u0002092\u0006\u0010\u0007\u001a\u00020:H\u0096@\u00a2\u0006\u0002\u0010;J\u0016\u0010<\u001a\u00020=2\u0006\u0010\u0007\u001a\u00020>H\u0096@\u00a2\u0006\u0002\u0010?J\u0016\u0010@\u001a\u00020A2\u0006\u0010\u0007\u001a\u00020BH\u0096@\u00a2\u0006\u0002\u0010CJ\u0016\u0010D\u001a\u00020E2\u0006\u0010\u0007\u001a\u00020FH\u0096@\u00a2\u0006\u0002\u0010GJ\u0016\u0010H\u001a\u00020I2\u0006\u0010\u0007\u001a\u00020JH\u0096@\u00a2\u0006\u0002\u0010KJ\u0016\u0010L\u001a\u00020M2\u0006\u0010\u0007\u001a\u00020NH\u0096@\u00a2\u0006\u0002\u0010OJ\u0016\u0010P\u001a\u00020Q2\u0006\u0010\u0007\u001a\u00020RH\u0096@\u00a2\u0006\u0002\u0010SJ\u0016\u0010T\u001a\u00020U2\u0006\u0010\u0007\u001a\u00020VH\u0096@\u00a2\u0006\u0002\u0010WJ\u0016\u0010X\u001a\u00020Y2\u0006\u0010\u0007\u001a\u00020ZH\u0096@\u00a2\u0006\u0002\u0010[J\u0016\u0010\\\u001a\u00020]2\u0006\u0010\u0007\u001a\u00020^H\u0096@\u00a2\u0006\u0002\u0010_J\u0016\u0010`\u001a\u00020a2\u0006\u0010\u0007\u001a\u00020bH\u0096@\u00a2\u0006\u0002\u0010cJ\u0016\u0010d\u001a\u00020e2\u0006\u0010\u0007\u001a\u00020fH\u0096@\u00a2\u0006\u0002\u0010gJ\u0016\u0010h\u001a\u00020i2\u0006\u0010\u0007\u001a\u00020jH\u0096@\u00a2\u0006\u0002\u0010kJ\u0016\u0010l\u001a\u00020m2\u0006\u0010\u0007\u001a\u00020nH\u0096@\u00a2\u0006\u0002\u0010oJ\u0016\u0010p\u001a\u00020q2\u0006\u0010\u0007\u001a\u00020rH\u0096@\u00a2\u0006\u0002\u0010sJ\u001c\u0010t\u001a\b\u0012\u0004\u0012\u00020v0u2\f\u0010w\u001a\b\u0012\u0004\u0012\u00020x0uH\u0016J\u0016\u0010y\u001a\u00020z2\u0006\u0010\u0007\u001a\u00020{H\u0096@\u00a2\u0006\u0002\u0010|J\u0017\u0010}\u001a\u00020~2\u0006\u0010\u0007\u001a\u00020\u007fH\u0096@\u00a2\u0006\u0003\u0010\u0080\u0001J\u001a\u0010\u0081\u0001\u001a\u00030\u0082\u00012\u0007\u0010\u0007\u001a\u00030\u0083\u0001H\u0096@\u00a2\u0006\u0003\u0010\u0084\u0001J\u001a\u0010\u0085\u0001\u001a\u00030\u0086\u00012\u0007\u0010\u0007\u001a\u00030\u0087\u0001H\u0096@\u00a2\u0006\u0003\u0010\u0088\u0001J\u001a\u0010\u0089\u0001\u001a\u00030\u008a\u00012\u0007\u0010\u0007\u001a\u00030\u008b\u0001H\u0096@\u00a2\u0006\u0003\u0010\u008c\u0001J\u001a\u0010\u008d\u0001\u001a\u00030\u008e\u00012\u0007\u0010\u0007\u001a\u00030\u008f\u0001H\u0096@\u00a2\u0006\u0003\u0010\u0090\u0001J\u001a\u0010\u0091\u0001\u001a\u00030\u0092\u00012\u0007\u0010\u0007\u001a\u00030\u0093\u0001H\u0096@\u00a2\u0006\u0003\u0010\u0094\u0001J\u001a\u0010\u0095\u0001\u001a\u00030\u0096\u00012\u0007\u0010\u0007\u001a\u00030\u0097\u0001H\u0096@\u00a2\u0006\u0003\u0010\u0098\u0001J\u001a\u0010\u0099\u0001\u001a\u00030\u009a\u00012\u0007\u0010\u0007\u001a\u00030\u009b\u0001H\u0096@\u00a2\u0006\u0003\u0010\u009c\u0001J\u001a\u0010\u009d\u0001\u001a\u00030\u009e\u00012\u0007\u0010\u0007\u001a\u00030\u009f\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00a0\u0001J\u001a\u0010\u00a1\u0001\u001a\u00030\u00a2\u00012\u0007\u0010\u0007\u001a\u00030\u00a3\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00a4\u0001J\u001a\u0010\u00a5\u0001\u001a\u00030\u00a6\u00012\u0007\u0010\u0007\u001a\u00030\u00a7\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00a8\u0001J\u001a\u0010\u00a9\u0001\u001a\u00030\u00aa\u00012\u0007\u0010\u0007\u001a\u00030\u00ab\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00ac\u0001J\u001a\u0010\u00ad\u0001\u001a\u00030\u00ae\u00012\u0007\u0010\u0007\u001a\u00030\u00af\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00b0\u0001J\u001a\u0010\u00b1\u0001\u001a\u00030\u00b2\u00012\u0007\u0010\u0007\u001a\u00030\u00b3\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00b4\u0001J\u001a\u0010\u00b5\u0001\u001a\u00030\u00b6\u00012\u0007\u0010\u0007\u001a\u00030\u00b7\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00b8\u0001J\u001a\u0010\u00b9\u0001\u001a\u00030\u00ba\u00012\u0007\u0010\u0007\u001a\u00030\u00bb\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00bc\u0001J\u001a\u0010\u00bd\u0001\u001a\u00030\u00be\u00012\u0007\u0010\u0007\u001a\u00030\u00bf\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00c0\u0001J\u001a\u0010\u00c1\u0001\u001a\u00030\u00c2\u00012\u0007\u0010\u0007\u001a\u00030\u00c3\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00c4\u0001J\u0019\u0010\u00c5\u0001\u001a\t\u0012\u0005\u0012\u00030\u00c6\u00010u2\u0007\u0010\u0007\u001a\u00030\u00c7\u0001H\u0016J\u0019\u0010\u00c8\u0001\u001a\t\u0012\u0005\u0012\u00030\u00c9\u00010u2\u0007\u0010\u0007\u001a\u00030\u00c7\u0001H\u0016J\u001a\u0010\u00ca\u0001\u001a\u00030\u00cb\u00012\u0007\u0010\u0007\u001a\u00030\u00cc\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00cd\u0001J\u001a\u0010\u00ce\u0001\u001a\u00030\u00cf\u00012\u0007\u0010\u0007\u001a\u00030\u00d0\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00d1\u0001J\u001a\u0010\u00d2\u0001\u001a\u00030\u00d3\u00012\u0007\u0010\u0007\u001a\u00030\u00d4\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00d5\u0001J\u001a\u0010\u00d6\u0001\u001a\u00030\u00d7\u00012\u0007\u0010\u0007\u001a\u00030\u00d8\u0001H\u0096@\u00a2\u0006\u0003\u0010\u00d9\u0001\u00a8\u0006\u00da\u0001"}, d2 = {"Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineImplBase;", "Lio/grpc/kotlin/AbstractCoroutineServerImpl;", "coroutineContext", "Lkotlin/coroutines/CoroutineContext;", "(Lkotlin/coroutines/CoroutineContext;)V", "acceptFriendRequest", "Lcom/tron/bridge/Friends$AcceptFriendRequestResponse;", "request", "Lcom/tron/bridge/Friends$AcceptFriendRequestRequest;", "(Lcom/tron/bridge/Friends$AcceptFriendRequestRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "acceptTeamInvite", "Lcom/tron/bridge/Teams$AcceptTeamInviteResponse;", "Lcom/tron/bridge/Teams$AcceptTeamInviteRequest;", "(Lcom/tron/bridge/Teams$AcceptTeamInviteRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "bindService", "Lio/grpc/ServerServiceDefinition;", "buyItem", "Lcom/tron/bridge/Shop$BuyItemResponse;", "Lcom/tron/bridge/Shop$BuyItemRequest;", "(Lcom/tron/bridge/Shop$BuyItemRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "buyPrefix", "Lcom/tron/bridge/Prefix$BuyPrefixResponse;", "Lcom/tron/bridge/Prefix$BuyPrefixRequest;", "(Lcom/tron/bridge/Prefix$BuyPrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "coinsLeaderboard", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$CoinsLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "createPrefix", "Lcom/tron/bridge/Prefix$CreatePrefixResponse;", "Lcom/tron/bridge/Prefix$CreatePrefixRequest;", "(Lcom/tron/bridge/Prefix$CreatePrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "createTeam", "Lcom/tron/bridge/Teams$CreateTeamResponse;", "Lcom/tron/bridge/Teams$CreateTeamRequest;", "(Lcom/tron/bridge/Teams$CreateTeamRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deathsLeaderboard", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$DeathsLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deletePrefix", "Lcom/tron/bridge/Prefix$DeletePrefixResponse;", "Lcom/tron/bridge/Prefix$DeletePrefixRequest;", "(Lcom/tron/bridge/Prefix$DeletePrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deleteTeam", "Lcom/tron/bridge/Teams$DeleteTeamResponse;", "Lcom/tron/bridge/Teams$DeleteTeamRequest;", "(Lcom/tron/bridge/Teams$DeleteTeamRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getAllPrefix", "Lcom/tron/bridge/Prefix$GetAllPrefixResponse;", "Lcom/tron/bridge/Prefix$GetAllPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetAllPrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getBalance", "Lcom/tron/bridge/Balance$GetBalanceResponse;", "Lcom/tron/bridge/Balance$GetBalanceRequest;", "(Lcom/tron/bridge/Balance$GetBalanceRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getCurrentPrefix", "Lcom/tron/bridge/Prefix$GetCurrentPrefixResponse;", "Lcom/tron/bridge/Prefix$GetCurrentPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetCurrentPrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getFriendRequests", "Lcom/tron/bridge/Friends$GetFriendRequestsResponse;", "Lcom/tron/bridge/Friends$GetFriendRequestsRequest;", "(Lcom/tron/bridge/Friends$GetFriendRequestsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getFriends", "Lcom/tron/bridge/Friends$GetFriendsResponse;", "Lcom/tron/bridge/Friends$GetFriendsRequest;", "(Lcom/tron/bridge/Friends$GetFriendsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getItems", "Lcom/tron/bridge/Shop$GetItemsResponse;", "Lcom/tron/bridge/Shop$GetItemsRequest;", "(Lcom/tron/bridge/Shop$GetItemsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getOpenTeams", "Lcom/tron/bridge/Teams$GetOpenTeamsResponse;", "Lcom/tron/bridge/Teams$GetOpenTeamsRequest;", "(Lcom/tron/bridge/Teams$GetOpenTeamsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getOwnedPrefix", "Lcom/tron/bridge/Prefix$GetOwnedPrefixResponse;", "Lcom/tron/bridge/Prefix$GetOwnedPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetOwnedPrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getTeamMembers", "Lcom/tron/bridge/Teams$GetTeamMembersResponse;", "Lcom/tron/bridge/Teams$GetTeamMembersRequest;", "(Lcom/tron/bridge/Teams$GetTeamMembersRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "joinTeam", "Lcom/tron/bridge/Teams$JoinTeamResponse;", "Lcom/tron/bridge/Teams$JoinTeamRequest;", "(Lcom/tron/bridge/Teams$JoinTeamRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "kdaLeaderboard", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$KdaLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "killsLeaderboard", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$KillsLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "leaveTeam", "Lcom/tron/bridge/Teams$LeaveTeamResponse;", "Lcom/tron/bridge/Teams$LeaveTeamRequest;", "(Lcom/tron/bridge/Teams$LeaveTeamRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "listFriendRequests", "Lcom/tron/bridge/Friends$ListFriendRequestsResponse;", "Lcom/tron/bridge/Friends$ListFriendRequestsRequest;", "(Lcom/tron/bridge/Friends$ListFriendRequestsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "listFriends", "Lcom/tron/bridge/Friends$ListFriendsResponse;", "Lcom/tron/bridge/Friends$ListFriendsRequest;", "(Lcom/tron/bridge/Friends$ListFriendsRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "lobbyShutdown", "Lcom/tron/bridge/Server$LobbyShutdownResponse;", "Lcom/tron/bridge/Server$LobbyShutdownRequest;", "(Lcom/tron/bridge/Server$LobbyShutdownRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "lobbyStartup", "Lcom/tron/bridge/Server$LobbyStartupResponse;", "Lcom/tron/bridge/Server$LobbyStartupRequest;", "(Lcom/tron/bridge/Server$LobbyStartupRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "message", "Lkotlinx/coroutines/flow/Flow;", "Lcom/tron/bridge/Server$MessageResponse;", "requests", "Lcom/tron/bridge/Server$MessageRequest;", "overallLeaderboard", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$OverallLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerBreakBlock", "Lcom/tron/bridge/Player$PlayerBreakBlockResponse;", "Lcom/tron/bridge/Player$PlayerBreakBlockRequest;", "(Lcom/tron/bridge/Player$PlayerBreakBlockRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerDeath", "Lcom/tron/bridge/Player$PlayerDeathResponse;", "Lcom/tron/bridge/Player$PlayerDeathRequest;", "(Lcom/tron/bridge/Player$PlayerDeathRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerJoin", "Lcom/tron/bridge/Session$PlayerJoinResponse;", "Lcom/tron/bridge/Session$PlayerJoinRequest;", "(Lcom/tron/bridge/Session$PlayerJoinRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerKill", "Lcom/tron/bridge/Player$PlayerKillResponse;", "Lcom/tron/bridge/Player$PlayerKillRequest;", "(Lcom/tron/bridge/Player$PlayerKillRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerLeave", "Lcom/tron/bridge/Session$PlayerLeaveResponse;", "Lcom/tron/bridge/Session$PlayerLeaveRequest;", "(Lcom/tron/bridge/Session$PlayerLeaveRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerPlaceBlock", "Lcom/tron/bridge/Player$PlayerPlaceBlockResponse;", "Lcom/tron/bridge/Player$PlayerPlaceBlockRequest;", "(Lcom/tron/bridge/Player$PlayerPlaceBlockRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "promoteTeamMember", "Lcom/tron/bridge/Teams$PromoteTeamMemberResponse;", "Lcom/tron/bridge/Teams$PromoteTeamMemberRequest;", "(Lcom/tron/bridge/Teams$PromoteTeamMemberRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxyShutdown", "Lcom/tron/bridge/Server$ProxyShutdownResponse;", "Lcom/tron/bridge/Server$ProxyShutdownRequest;", "(Lcom/tron/bridge/Server$ProxyShutdownRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxyStartup", "Lcom/tron/bridge/Server$ProxyStartupResponse;", "Lcom/tron/bridge/Server$ProxyStartupRequest;", "(Lcom/tron/bridge/Server$ProxyStartupRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "rejectFriendRequest", "Lcom/tron/bridge/Friends$RejectFriendRequestResponse;", "Lcom/tron/bridge/Friends$RejectFriendRequestRequest;", "(Lcom/tron/bridge/Friends$RejectFriendRequestRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "rejectTeamInvite", "Lcom/tron/bridge/Teams$RejectTeamInviteResponse;", "Lcom/tron/bridge/Teams$RejectTeamInviteRequest;", "(Lcom/tron/bridge/Teams$RejectTeamInviteRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "removeFriend", "Lcom/tron/bridge/Friends$RemoveFriendResponse;", "Lcom/tron/bridge/Friends$RemoveFriendRequest;", "(Lcom/tron/bridge/Friends$RemoveFriendRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "removeTeamMember", "Lcom/tron/bridge/Friends$RemoveTeamMemberResponse;", "Lcom/tron/bridge/Friends$RemoveTeamMemberRequest;", "(Lcom/tron/bridge/Friends$RemoveTeamMemberRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "reportPlayer", "Lcom/tron/bridge/Security$ReportPlayerResponse;", "Lcom/tron/bridge/Security$ReportPlayerRequest;", "(Lcom/tron/bridge/Security$ReportPlayerRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "selectPrefix", "Lcom/tron/bridge/Prefix$SelectPrefixResponse;", "Lcom/tron/bridge/Prefix$SelectPrefixRequest;", "(Lcom/tron/bridge/Prefix$SelectPrefixRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sellItem", "Lcom/tron/bridge/Shop$SellItemResponse;", "Lcom/tron/bridge/Shop$SellItemRequest;", "(Lcom/tron/bridge/Shop$SellItemRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sendFriendRequest", "Lcom/tron/bridge/Friends$SendFriendRequestResponse;", "Lcom/tron/bridge/Friends$SendFriendRequestRequest;", "(Lcom/tron/bridge/Friends$SendFriendRequestRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sendTeamInvite", "Lcom/tron/bridge/Teams$SendTeamInviteResponse;", "Lcom/tron/bridge/Teams$SendTeamInviteRequest;", "(Lcom/tron/bridge/Teams$SendTeamInviteRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "serverSendMessage", "Lcom/tron/bridge/Server$ServerSendMessageResponse;", "Lcom/tron/bridge/Server$ServerSubscribeRequest;", "serverSendTitle", "Lcom/tron/bridge/Server$ServerSendTitleResponse;", "survivalShutdown", "Lcom/tron/bridge/Server$SurvivalShutdownResponse;", "Lcom/tron/bridge/Server$SurvivalShutdownRequest;", "(Lcom/tron/bridge/Server$SurvivalShutdownRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "survivalStartup", "Lcom/tron/bridge/Server$SurvivalStartupResponse;", "Lcom/tron/bridge/Server$SurvivalStartupRequest;", "(Lcom/tron/bridge/Server$SurvivalStartupRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "teamsLeaderboard", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$TeamsLeaderboardRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "transferBalance", "Lcom/tron/bridge/Balance$TransferBalanceResponse;", "Lcom/tron/bridge/Balance$TransferBalanceRequest;", "(Lcom/tron/bridge/Balance$TransferBalanceRequest;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxy"})
    public static abstract class BridgeCoroutineImplBase extends io.grpc.kotlin.AbstractCoroutineServerImpl {
        
        public BridgeCoroutineImplBase(@org.jetbrains.annotations.NotNull()
        kotlin.coroutines.CoroutineContext coroutineContext) {
            super(null);
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerJoin(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Session.PlayerJoinRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Session.PlayerJoinResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerLeave(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Session.PlayerLeaveRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Session.PlayerLeaveResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getBalance(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Balance.GetBalanceRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Balance.GetBalanceResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object transferBalance(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Balance.TransferBalanceRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Balance.TransferBalanceResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object overallLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.OverallLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.OverallLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object coinsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object teamsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object kdaLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.KdaLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.KdaLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object deathsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object killsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.KillsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.KillsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getFriends(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.GetFriendsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.GetFriendsResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.ListFriends.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object listFriends(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.ListFriendsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.ListFriendsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object sendFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.SendFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.SendFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object acceptFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.AcceptFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.AcceptFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object rejectFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RejectFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RejectFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getFriendRequests(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.GetFriendRequestsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.GetFriendRequestsResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.ListFriendRequests.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object listFriendRequests(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.ListFriendRequestsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.ListFriendRequestsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object removeFriend(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RemoveFriendRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RemoveFriendResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object createTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.CreateTeamRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.CreateTeamResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.DeleteTeam.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object deleteTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.DeleteTeamRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.DeleteTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object leaveTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.LeaveTeamRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.LeaveTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object joinTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.JoinTeamRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.JoinTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object sendTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.SendTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.SendTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object acceptTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.AcceptTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.AcceptTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object rejectTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.RejectTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.RejectTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getTeamMembers(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.GetTeamMembersRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.GetTeamMembersResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object removeTeamMember(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RemoveTeamMemberRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RemoveTeamMemberResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object promoteTeamMember(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.PromoteTeamMemberRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.PromoteTeamMemberResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.GetOpenTeams.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getOpenTeams(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.GetOpenTeamsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.GetOpenTeamsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object buyItem(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.BuyItemRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.BuyItemResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object sellItem(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.SellItemRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.SellItemResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getItems(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.GetItemsRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.GetItemsResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.BuyPrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object buyPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.BuyPrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.BuyPrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.SelectPrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object selectPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.SelectPrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.SelectPrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.GetAllPrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getAllPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetAllPrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetAllPrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.GetOwnedPrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getOwnedPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetOwnedPrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetOwnedPrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.GetCurrentPrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object getCurrentPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetCurrentPrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetCurrentPrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.CreatePrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object createPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.CreatePrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.CreatePrefixResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.DeletePrefix.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object deletePrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.DeletePrefixRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.DeletePrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerDeath(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerDeathRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerDeathResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerKill(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerKillRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerKillResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.PlayerPlaceBlock.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerPlaceBlock(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerPlaceBlockRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerPlaceBlockResponse> $completion) {
            return null;
        }
        
        /**
         * Returns the response to an RPC for bridge.Bridge.PlayerBreakBlock.
         *
         * If this method fails with a [StatusException], the RPC will fail with the corresponding
         * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException],
         * the RPC will fail
         * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
         * fail with `Status.UNKNOWN` with the exception as a cause.
         *
         * @param request The request from the client.
         */
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object playerBreakBlock(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerBreakBlockRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerBreakBlockResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object proxyStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ProxyStartupRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.ProxyStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object proxyShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ProxyShutdownRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.ProxyShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object survivalStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.SurvivalStartupRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.SurvivalStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object survivalShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.SurvivalShutdownRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.SurvivalShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object lobbyStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.LobbyStartupRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.LobbyStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object lobbyShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.LobbyShutdownRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.LobbyShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.NotNull()
        public kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.ServerSendMessageResponse> serverSendMessage(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ServerSubscribeRequest request) {
            return null;
        }
        
        /**
         * Returns a [Flow] of responses to an RPC for bridge.Bridge.ServerSendTitle.
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
        @org.jetbrains.annotations.NotNull()
        public kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.ServerSendTitleResponse> serverSendTitle(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ServerSubscribeRequest request) {
            return null;
        }
        
        /**
         * Returns a [Flow] of responses to an RPC for bridge.Bridge.Message.
         *
         * If creating or collecting the returned flow fails with a [StatusException], the RPC
         * will fail with the corresponding [io.grpc.Status].  If it fails with a
         * [java.util.concurrent.CancellationException], the RPC will fail with status
         * `Status.CANCELLED`.  If creating
         * or collecting the returned flow fails for any other reason, the RPC will fail with
         * `Status.UNKNOWN` with the exception as a cause.
         *
         * @param requests A [Flow] of requests from the client.  This flow can be
         *       collected only once and throws [java.lang.IllegalStateException] on attempts to
         * collect
         *       it more than once.
         */
        @org.jetbrains.annotations.NotNull()
        public kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.MessageResponse> message(@org.jetbrains.annotations.NotNull()
        kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.MessageRequest> requests) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public java.lang.Object reportPlayer(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Security.ReportPlayerRequest request, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Security.ReportPlayerResponse> $completion) {
            return null;
        }
        
        @java.lang.Override()
        @org.jetbrains.annotations.NotNull()
        public final io.grpc.ServerServiceDefinition bindService() {
            return null;
        }
        
        public BridgeCoroutineImplBase() {
            super(null);
        }
    }
    
    /**
     * A stub for issuing RPCs to a(n) bridge.Bridge service as suspending coroutines.
     */
    @io.grpc.kotlin.StubFor(value = com.tron.bridge.BridgeGrpc.class)
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0098\u0005\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\b\u0007\u0018\u00002\b\u0012\u0004\u0012\u00020\u00000\u0001B\u0019\b\u0007\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\b\b\u0002\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J \u0010\u0007\u001a\u00020\b2\u0006\u0010\t\u001a\u00020\n2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\rJ \u0010\u000e\u001a\u00020\u000f2\u0006\u0010\t\u001a\u00020\u00102\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\u0011J\u0018\u0010\u0012\u001a\u00020\u00002\u0006\u0010\u0002\u001a\u00020\u00032\u0006\u0010\u0004\u001a\u00020\u0005H\u0014J \u0010\u0013\u001a\u00020\u00142\u0006\u0010\t\u001a\u00020\u00152\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\u0016J \u0010\u0017\u001a\u00020\u00182\u0006\u0010\t\u001a\u00020\u00192\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\u001aJ \u0010\u001b\u001a\u00020\u001c2\u0006\u0010\t\u001a\u00020\u001d2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\u001eJ \u0010\u001f\u001a\u00020 2\u0006\u0010\t\u001a\u00020!2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\"J \u0010#\u001a\u00020$2\u0006\u0010\t\u001a\u00020%2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010&J \u0010\'\u001a\u00020(2\u0006\u0010\t\u001a\u00020)2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010*J \u0010+\u001a\u00020,2\u0006\u0010\t\u001a\u00020-2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010.J \u0010/\u001a\u0002002\u0006\u0010\t\u001a\u0002012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u00102J \u00103\u001a\u0002042\u0006\u0010\t\u001a\u0002052\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u00106J \u00107\u001a\u0002082\u0006\u0010\t\u001a\u0002092\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010:J \u0010;\u001a\u00020<2\u0006\u0010\t\u001a\u00020=2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010>J \u0010?\u001a\u00020@2\u0006\u0010\t\u001a\u00020A2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010BJ \u0010C\u001a\u00020D2\u0006\u0010\t\u001a\u00020E2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010FJ \u0010G\u001a\u00020H2\u0006\u0010\t\u001a\u00020I2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010JJ \u0010K\u001a\u00020L2\u0006\u0010\t\u001a\u00020M2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010NJ \u0010O\u001a\u00020P2\u0006\u0010\t\u001a\u00020Q2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010RJ \u0010S\u001a\u00020T2\u0006\u0010\t\u001a\u00020U2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010VJ \u0010W\u001a\u00020X2\u0006\u0010\t\u001a\u00020Y2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010ZJ \u0010[\u001a\u00020\\2\u0006\u0010\t\u001a\u00020]2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010^J \u0010_\u001a\u00020`2\u0006\u0010\t\u001a\u00020a2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010bJ \u0010c\u001a\u00020d2\u0006\u0010\t\u001a\u00020e2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010fJ \u0010g\u001a\u00020h2\u0006\u0010\t\u001a\u00020i2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010jJ \u0010k\u001a\u00020l2\u0006\u0010\t\u001a\u00020m2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010nJ \u0010o\u001a\u00020p2\u0006\u0010\t\u001a\u00020q2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010rJ \u0010s\u001a\u00020t2\u0006\u0010\t\u001a\u00020u2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010vJ$\u0010w\u001a\b\u0012\u0004\u0012\u00020y0x2\f\u0010z\u001a\b\u0012\u0004\u0012\u00020{0x2\b\b\u0002\u0010\u000b\u001a\u00020\fJ \u0010|\u001a\u00020}2\u0006\u0010\t\u001a\u00020~2\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0002\u0010\u007fJ$\u0010\u0080\u0001\u001a\u00030\u0081\u00012\u0007\u0010\t\u001a\u00030\u0082\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u0083\u0001J$\u0010\u0084\u0001\u001a\u00030\u0085\u00012\u0007\u0010\t\u001a\u00030\u0086\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u0087\u0001J$\u0010\u0088\u0001\u001a\u00030\u0089\u00012\u0007\u0010\t\u001a\u00030\u008a\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u008b\u0001J$\u0010\u008c\u0001\u001a\u00030\u008d\u00012\u0007\u0010\t\u001a\u00030\u008e\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u008f\u0001J$\u0010\u0090\u0001\u001a\u00030\u0091\u00012\u0007\u0010\t\u001a\u00030\u0092\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u0093\u0001J$\u0010\u0094\u0001\u001a\u00030\u0095\u00012\u0007\u0010\t\u001a\u00030\u0096\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u0097\u0001J$\u0010\u0098\u0001\u001a\u00030\u0099\u00012\u0007\u0010\t\u001a\u00030\u009a\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u009b\u0001J$\u0010\u009c\u0001\u001a\u00030\u009d\u00012\u0007\u0010\t\u001a\u00030\u009e\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u009f\u0001J$\u0010\u00a0\u0001\u001a\u00030\u00a1\u00012\u0007\u0010\t\u001a\u00030\u00a2\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00a3\u0001J$\u0010\u00a4\u0001\u001a\u00030\u00a5\u00012\u0007\u0010\t\u001a\u00030\u00a6\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00a7\u0001J$\u0010\u00a8\u0001\u001a\u00030\u00a9\u00012\u0007\u0010\t\u001a\u00030\u00aa\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00ab\u0001J$\u0010\u00ac\u0001\u001a\u00030\u00ad\u00012\u0007\u0010\t\u001a\u00030\u00ae\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00af\u0001J$\u0010\u00b0\u0001\u001a\u00030\u00b1\u00012\u0007\u0010\t\u001a\u00030\u00b2\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00b3\u0001J$\u0010\u00b4\u0001\u001a\u00030\u00b5\u00012\u0007\u0010\t\u001a\u00030\u00b6\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00b7\u0001J$\u0010\u00b8\u0001\u001a\u00030\u00b9\u00012\u0007\u0010\t\u001a\u00030\u00ba\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00bb\u0001J$\u0010\u00bc\u0001\u001a\u00030\u00bd\u00012\u0007\u0010\t\u001a\u00030\u00be\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00bf\u0001J$\u0010\u00c0\u0001\u001a\u00030\u00c1\u00012\u0007\u0010\t\u001a\u00030\u00c2\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00c3\u0001J$\u0010\u00c4\u0001\u001a\u00030\u00c5\u00012\u0007\u0010\t\u001a\u00030\u00c6\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00c7\u0001J!\u0010\u00c8\u0001\u001a\t\u0012\u0005\u0012\u00030\u00c9\u00010x2\u0007\u0010\t\u001a\u00030\u00ca\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fJ!\u0010\u00cb\u0001\u001a\t\u0012\u0005\u0012\u00030\u00cc\u00010x2\u0007\u0010\t\u001a\u00030\u00ca\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fJ$\u0010\u00cd\u0001\u001a\u00030\u00ce\u00012\u0007\u0010\t\u001a\u00030\u00cf\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00d0\u0001J$\u0010\u00d1\u0001\u001a\u00030\u00d2\u00012\u0007\u0010\t\u001a\u00030\u00d3\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00d4\u0001J$\u0010\u00d5\u0001\u001a\u00030\u00d6\u00012\u0007\u0010\t\u001a\u00030\u00d7\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00d8\u0001J$\u0010\u00d9\u0001\u001a\u00030\u00da\u00012\u0007\u0010\t\u001a\u00030\u00db\u00012\b\b\u0002\u0010\u000b\u001a\u00020\fH\u0086@\u00a2\u0006\u0003\u0010\u00dc\u0001\u00a8\u0006\u00dd\u0001"}, d2 = {"Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "Lio/grpc/kotlin/AbstractCoroutineStub;", "channel", "Lio/grpc/Channel;", "callOptions", "Lio/grpc/CallOptions;", "(Lio/grpc/Channel;Lio/grpc/CallOptions;)V", "acceptFriendRequest", "Lcom/tron/bridge/Friends$AcceptFriendRequestResponse;", "request", "Lcom/tron/bridge/Friends$AcceptFriendRequestRequest;", "headers", "Lio/grpc/Metadata;", "(Lcom/tron/bridge/Friends$AcceptFriendRequestRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "acceptTeamInvite", "Lcom/tron/bridge/Teams$AcceptTeamInviteResponse;", "Lcom/tron/bridge/Teams$AcceptTeamInviteRequest;", "(Lcom/tron/bridge/Teams$AcceptTeamInviteRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "build", "buyItem", "Lcom/tron/bridge/Shop$BuyItemResponse;", "Lcom/tron/bridge/Shop$BuyItemRequest;", "(Lcom/tron/bridge/Shop$BuyItemRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "buyPrefix", "Lcom/tron/bridge/Prefix$BuyPrefixResponse;", "Lcom/tron/bridge/Prefix$BuyPrefixRequest;", "(Lcom/tron/bridge/Prefix$BuyPrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "coinsLeaderboard", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$CoinsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$CoinsLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "createPrefix", "Lcom/tron/bridge/Prefix$CreatePrefixResponse;", "Lcom/tron/bridge/Prefix$CreatePrefixRequest;", "(Lcom/tron/bridge/Prefix$CreatePrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "createTeam", "Lcom/tron/bridge/Teams$CreateTeamResponse;", "Lcom/tron/bridge/Teams$CreateTeamRequest;", "(Lcom/tron/bridge/Teams$CreateTeamRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deathsLeaderboard", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$DeathsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$DeathsLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deletePrefix", "Lcom/tron/bridge/Prefix$DeletePrefixResponse;", "Lcom/tron/bridge/Prefix$DeletePrefixRequest;", "(Lcom/tron/bridge/Prefix$DeletePrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deleteTeam", "Lcom/tron/bridge/Teams$DeleteTeamResponse;", "Lcom/tron/bridge/Teams$DeleteTeamRequest;", "(Lcom/tron/bridge/Teams$DeleteTeamRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getAllPrefix", "Lcom/tron/bridge/Prefix$GetAllPrefixResponse;", "Lcom/tron/bridge/Prefix$GetAllPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetAllPrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getBalance", "Lcom/tron/bridge/Balance$GetBalanceResponse;", "Lcom/tron/bridge/Balance$GetBalanceRequest;", "(Lcom/tron/bridge/Balance$GetBalanceRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getCurrentPrefix", "Lcom/tron/bridge/Prefix$GetCurrentPrefixResponse;", "Lcom/tron/bridge/Prefix$GetCurrentPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetCurrentPrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getFriendRequests", "Lcom/tron/bridge/Friends$GetFriendRequestsResponse;", "Lcom/tron/bridge/Friends$GetFriendRequestsRequest;", "(Lcom/tron/bridge/Friends$GetFriendRequestsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getFriends", "Lcom/tron/bridge/Friends$GetFriendsResponse;", "Lcom/tron/bridge/Friends$GetFriendsRequest;", "(Lcom/tron/bridge/Friends$GetFriendsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getItems", "Lcom/tron/bridge/Shop$GetItemsResponse;", "Lcom/tron/bridge/Shop$GetItemsRequest;", "(Lcom/tron/bridge/Shop$GetItemsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getOpenTeams", "Lcom/tron/bridge/Teams$GetOpenTeamsResponse;", "Lcom/tron/bridge/Teams$GetOpenTeamsRequest;", "(Lcom/tron/bridge/Teams$GetOpenTeamsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getOwnedPrefix", "Lcom/tron/bridge/Prefix$GetOwnedPrefixResponse;", "Lcom/tron/bridge/Prefix$GetOwnedPrefixRequest;", "(Lcom/tron/bridge/Prefix$GetOwnedPrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "getTeamMembers", "Lcom/tron/bridge/Teams$GetTeamMembersResponse;", "Lcom/tron/bridge/Teams$GetTeamMembersRequest;", "(Lcom/tron/bridge/Teams$GetTeamMembersRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "joinTeam", "Lcom/tron/bridge/Teams$JoinTeamResponse;", "Lcom/tron/bridge/Teams$JoinTeamRequest;", "(Lcom/tron/bridge/Teams$JoinTeamRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "kdaLeaderboard", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$KdaLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$KdaLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "killsLeaderboard", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$KillsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$KillsLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "leaveTeam", "Lcom/tron/bridge/Teams$LeaveTeamResponse;", "Lcom/tron/bridge/Teams$LeaveTeamRequest;", "(Lcom/tron/bridge/Teams$LeaveTeamRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "listFriendRequests", "Lcom/tron/bridge/Friends$ListFriendRequestsResponse;", "Lcom/tron/bridge/Friends$ListFriendRequestsRequest;", "(Lcom/tron/bridge/Friends$ListFriendRequestsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "listFriends", "Lcom/tron/bridge/Friends$ListFriendsResponse;", "Lcom/tron/bridge/Friends$ListFriendsRequest;", "(Lcom/tron/bridge/Friends$ListFriendsRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "lobbyShutdown", "Lcom/tron/bridge/Server$LobbyShutdownResponse;", "Lcom/tron/bridge/Server$LobbyShutdownRequest;", "(Lcom/tron/bridge/Server$LobbyShutdownRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "lobbyStartup", "Lcom/tron/bridge/Server$LobbyStartupResponse;", "Lcom/tron/bridge/Server$LobbyStartupRequest;", "(Lcom/tron/bridge/Server$LobbyStartupRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "message", "Lkotlinx/coroutines/flow/Flow;", "Lcom/tron/bridge/Server$MessageResponse;", "requests", "Lcom/tron/bridge/Server$MessageRequest;", "overallLeaderboard", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$OverallLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$OverallLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerBreakBlock", "Lcom/tron/bridge/Player$PlayerBreakBlockResponse;", "Lcom/tron/bridge/Player$PlayerBreakBlockRequest;", "(Lcom/tron/bridge/Player$PlayerBreakBlockRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerDeath", "Lcom/tron/bridge/Player$PlayerDeathResponse;", "Lcom/tron/bridge/Player$PlayerDeathRequest;", "(Lcom/tron/bridge/Player$PlayerDeathRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerJoin", "Lcom/tron/bridge/Session$PlayerJoinResponse;", "Lcom/tron/bridge/Session$PlayerJoinRequest;", "(Lcom/tron/bridge/Session$PlayerJoinRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerKill", "Lcom/tron/bridge/Player$PlayerKillResponse;", "Lcom/tron/bridge/Player$PlayerKillRequest;", "(Lcom/tron/bridge/Player$PlayerKillRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerLeave", "Lcom/tron/bridge/Session$PlayerLeaveResponse;", "Lcom/tron/bridge/Session$PlayerLeaveRequest;", "(Lcom/tron/bridge/Session$PlayerLeaveRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "playerPlaceBlock", "Lcom/tron/bridge/Player$PlayerPlaceBlockResponse;", "Lcom/tron/bridge/Player$PlayerPlaceBlockRequest;", "(Lcom/tron/bridge/Player$PlayerPlaceBlockRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "promoteTeamMember", "Lcom/tron/bridge/Teams$PromoteTeamMemberResponse;", "Lcom/tron/bridge/Teams$PromoteTeamMemberRequest;", "(Lcom/tron/bridge/Teams$PromoteTeamMemberRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxyShutdown", "Lcom/tron/bridge/Server$ProxyShutdownResponse;", "Lcom/tron/bridge/Server$ProxyShutdownRequest;", "(Lcom/tron/bridge/Server$ProxyShutdownRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxyStartup", "Lcom/tron/bridge/Server$ProxyStartupResponse;", "Lcom/tron/bridge/Server$ProxyStartupRequest;", "(Lcom/tron/bridge/Server$ProxyStartupRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "rejectFriendRequest", "Lcom/tron/bridge/Friends$RejectFriendRequestResponse;", "Lcom/tron/bridge/Friends$RejectFriendRequestRequest;", "(Lcom/tron/bridge/Friends$RejectFriendRequestRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "rejectTeamInvite", "Lcom/tron/bridge/Teams$RejectTeamInviteResponse;", "Lcom/tron/bridge/Teams$RejectTeamInviteRequest;", "(Lcom/tron/bridge/Teams$RejectTeamInviteRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "removeFriend", "Lcom/tron/bridge/Friends$RemoveFriendResponse;", "Lcom/tron/bridge/Friends$RemoveFriendRequest;", "(Lcom/tron/bridge/Friends$RemoveFriendRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "removeTeamMember", "Lcom/tron/bridge/Friends$RemoveTeamMemberResponse;", "Lcom/tron/bridge/Friends$RemoveTeamMemberRequest;", "(Lcom/tron/bridge/Friends$RemoveTeamMemberRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "reportPlayer", "Lcom/tron/bridge/Security$ReportPlayerResponse;", "Lcom/tron/bridge/Security$ReportPlayerRequest;", "(Lcom/tron/bridge/Security$ReportPlayerRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "selectPrefix", "Lcom/tron/bridge/Prefix$SelectPrefixResponse;", "Lcom/tron/bridge/Prefix$SelectPrefixRequest;", "(Lcom/tron/bridge/Prefix$SelectPrefixRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sellItem", "Lcom/tron/bridge/Shop$SellItemResponse;", "Lcom/tron/bridge/Shop$SellItemRequest;", "(Lcom/tron/bridge/Shop$SellItemRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sendFriendRequest", "Lcom/tron/bridge/Friends$SendFriendRequestResponse;", "Lcom/tron/bridge/Friends$SendFriendRequestRequest;", "(Lcom/tron/bridge/Friends$SendFriendRequestRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sendTeamInvite", "Lcom/tron/bridge/Teams$SendTeamInviteResponse;", "Lcom/tron/bridge/Teams$SendTeamInviteRequest;", "(Lcom/tron/bridge/Teams$SendTeamInviteRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "serverSendMessage", "Lcom/tron/bridge/Server$ServerSendMessageResponse;", "Lcom/tron/bridge/Server$ServerSubscribeRequest;", "serverSendTitle", "Lcom/tron/bridge/Server$ServerSendTitleResponse;", "survivalShutdown", "Lcom/tron/bridge/Server$SurvivalShutdownResponse;", "Lcom/tron/bridge/Server$SurvivalShutdownRequest;", "(Lcom/tron/bridge/Server$SurvivalShutdownRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "survivalStartup", "Lcom/tron/bridge/Server$SurvivalStartupResponse;", "Lcom/tron/bridge/Server$SurvivalStartupRequest;", "(Lcom/tron/bridge/Server$SurvivalStartupRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "teamsLeaderboard", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardResponse;", "Lcom/tron/bridge/Leaderboard$TeamsLeaderboardRequest;", "(Lcom/tron/bridge/Leaderboard$TeamsLeaderboardRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "transferBalance", "Lcom/tron/bridge/Balance$TransferBalanceResponse;", "Lcom/tron/bridge/Balance$TransferBalanceRequest;", "(Lcom/tron/bridge/Balance$TransferBalanceRequest;Lio/grpc/Metadata;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "proxy"})
    public static final class BridgeCoroutineStub extends io.grpc.kotlin.AbstractCoroutineStub<com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub> {
        
        @kotlin.jvm.JvmOverloads()
        public BridgeCoroutineStub(@org.jetbrains.annotations.NotNull()
        io.grpc.Channel channel) {
            super(null, null);
        }
        
        @kotlin.jvm.JvmOverloads()
        public BridgeCoroutineStub(@org.jetbrains.annotations.NotNull()
        io.grpc.Channel channel, @org.jetbrains.annotations.NotNull()
        io.grpc.CallOptions callOptions) {
            super(null, null);
        }
        
        @java.lang.Override()
        @org.jetbrains.annotations.NotNull()
        protected com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub build(@org.jetbrains.annotations.NotNull()
        io.grpc.Channel channel, @org.jetbrains.annotations.NotNull()
        io.grpc.CallOptions callOptions) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerJoin(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Session.PlayerJoinRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Session.PlayerJoinResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerLeave(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Session.PlayerLeaveRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Session.PlayerLeaveResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getBalance(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Balance.GetBalanceRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Balance.GetBalanceResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object transferBalance(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Balance.TransferBalanceRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Balance.TransferBalanceResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object overallLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.OverallLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.OverallLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object coinsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object teamsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object kdaLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.KdaLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.KdaLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object deathsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object killsLeaderboard(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.KillsLeaderboardRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Leaderboard.KillsLeaderboardResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getFriends(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.GetFriendsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.GetFriendsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object listFriends(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.ListFriendsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.ListFriendsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object sendFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.SendFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.SendFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object acceptFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.AcceptFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.AcceptFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object rejectFriendRequest(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RejectFriendRequestRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RejectFriendRequestResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getFriendRequests(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.GetFriendRequestsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.GetFriendRequestsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object listFriendRequests(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.ListFriendRequestsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.ListFriendRequestsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object removeFriend(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RemoveFriendRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RemoveFriendResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object createTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.CreateTeamRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.CreateTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object deleteTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.DeleteTeamRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.DeleteTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object leaveTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.LeaveTeamRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.LeaveTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object joinTeam(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.JoinTeamRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.JoinTeamResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object sendTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.SendTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.SendTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object acceptTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.AcceptTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.AcceptTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object rejectTeamInvite(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.RejectTeamInviteRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.RejectTeamInviteResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getTeamMembers(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.GetTeamMembersRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.GetTeamMembersResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object removeTeamMember(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Friends.RemoveTeamMemberRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Friends.RemoveTeamMemberResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object promoteTeamMember(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.PromoteTeamMemberRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.PromoteTeamMemberResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getOpenTeams(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Teams.GetOpenTeamsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Teams.GetOpenTeamsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object buyItem(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.BuyItemRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.BuyItemResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object sellItem(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.SellItemRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.SellItemResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getItems(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Shop.GetItemsRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Shop.GetItemsResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object buyPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.BuyPrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.BuyPrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object selectPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.SelectPrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.SelectPrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getAllPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetAllPrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetAllPrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getOwnedPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetOwnedPrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetOwnedPrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object getCurrentPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.GetCurrentPrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.GetCurrentPrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object createPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.CreatePrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.CreatePrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object deletePrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Prefix.DeletePrefixRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Prefix.DeletePrefixResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerDeath(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerDeathRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerDeathResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerKill(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerKillRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerKillResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerPlaceBlock(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerPlaceBlockRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerPlaceBlockResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object playerBreakBlock(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Player.PlayerBreakBlockRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Player.PlayerBreakBlockResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object proxyStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ProxyStartupRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.ProxyStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object proxyShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ProxyShutdownRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.ProxyShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object survivalStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.SurvivalStartupRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.SurvivalStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object survivalShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.SurvivalShutdownRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.SurvivalShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object lobbyStartup(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.LobbyStartupRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.LobbyStartupResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object lobbyShutdown(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.LobbyShutdownRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Server.LobbyShutdownResponse> $completion) {
            return null;
        }
        
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
        @org.jetbrains.annotations.NotNull()
        public final kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.ServerSendMessageResponse> serverSendMessage(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ServerSubscribeRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers) {
            return null;
        }
        
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
        @org.jetbrains.annotations.NotNull()
        public final kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.ServerSendTitleResponse> serverSendTitle(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Server.ServerSubscribeRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers) {
            return null;
        }
        
        /**
         * Returns a [Flow] that, when collected, executes this RPC and emits responses from the
         * server as they arrive.  That flow finishes normally if the server closes its response with
         * [`Status.OK`][io.grpc.Status], and fails by throwing a [StatusException] otherwise.  If
         * collecting the flow downstream fails exceptionally (including via cancellation), the RPC
         * is cancelled with that exception as a cause.
         *
         * The [Flow] of requests is collected once each time the [Flow] of responses is
         * collected. If collection of the [Flow] of responses completes normally or
         * exceptionally before collection of `requests` completes, the collection of
         * `requests` is cancelled.  If the collection of `requests` completes
         * exceptionally for any other reason, then the collection of the [Flow] of responses
         * completes exceptionally for the same reason and the RPC is cancelled with that reason.
         *
         * @param requests A [Flow] of request messages.
         *
         * @param headers Metadata to attach to the request.  Most users will not need this.
         *
         * @return A flow that, when collected, emits the responses from the server.
         */
        @org.jetbrains.annotations.NotNull()
        public final kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.MessageResponse> message(@org.jetbrains.annotations.NotNull()
        kotlinx.coroutines.flow.Flow<com.tron.bridge.Server.MessageRequest> requests, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers) {
            return null;
        }
        
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
        @org.jetbrains.annotations.Nullable()
        public final java.lang.Object reportPlayer(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Security.ReportPlayerRequest request, @org.jetbrains.annotations.NotNull()
        io.grpc.Metadata headers, @org.jetbrains.annotations.NotNull()
        kotlin.coroutines.Continuation<? super com.tron.bridge.Security.ReportPlayerResponse> $completion) {
            return null;
        }
    }
}