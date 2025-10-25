package com.tron.bridge;

/**
 * Protobuf type `bridge.GetLeaderboardResponse`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/GetLeaderboardResponseKt;", "", "()V", "Dsl", "proxy"})
public final class GetLeaderboardResponseKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.GetLeaderboardResponseKt INSTANCE = null;
    
    private GetLeaderboardResponseKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000<\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0010\u000e\n\u0002\u0010\t\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0007\n\u0002\u0010$\n\u0002\b\b\b\u0007\u0018\u0000 \u001d2\u00020\u0001:\u0002\u001d\u001eB\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\f\u001a\u00020\rH\u0001J#\u0010\u000e\u001a\u00020\u000f*\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u0006H\u0007\u00a2\u0006\u0002\b\u0010J3\u0010\u0011\u001a\u00020\u000f*\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u00062\u0006\u0010\u0012\u001a\u00020\u00072\u0006\u0010\u0013\u001a\u00020\bH\u0007\u00a2\u0006\u0002\b\u0014J7\u0010\u0015\u001a\u00020\u000f*\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u00062\u0012\u0010\u0016\u001a\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u0017H\u0007\u00a2\u0006\u0002\b\u0018J+\u0010\u0019\u001a\u00020\u000f*\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u00062\u0006\u0010\u0012\u001a\u00020\u0007H\u0007\u00a2\u0006\u0002\b\u001aJ4\u0010\u001b\u001a\u00020\u000f*\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u00062\u0006\u0010\u0012\u001a\u00020\u00072\u0006\u0010\u0013\u001a\u00020\bH\u0087\n\u00a2\u0006\u0002\b\u001cR\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R#\u0010\u0005\u001a\u0014\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b\u0012\u0004\u0012\u00020\t0\u00068G\u00a2\u0006\u0006\u001a\u0004\b\n\u0010\u000b\u00a8\u0006\u001f"}, d2 = {"Lcom/tron/bridge/GetLeaderboardResponseKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Leaderboard$GetLeaderboardResponse$Builder;", "(Lcom/tron/bridge/Leaderboard$GetLeaderboardResponse$Builder;)V", "players", "Lcom/google/protobuf/kotlin/DslMap;", "", "", "Lcom/tron/bridge/GetLeaderboardResponseKt$Dsl$PlayersProxy;", "getPlayersMap", "()Lcom/google/protobuf/kotlin/DslMap;", "_build", "Lcom/tron/bridge/Leaderboard$GetLeaderboardResponse;", "clear", "", "clearPlayers", "put", "key", "value", "putPlayers", "putAll", "map", "", "putAllPlayers", "remove", "removePlayers", "set", "setPlayers", "Companion", "PlayersProxy", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Leaderboard.GetLeaderboardResponse.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.GetLeaderboardResponseKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Leaderboard.GetLeaderboardResponse.Builder _builder) {
            super();
        }
        
        /**
         * `map<string, uint64> players = 1;`
         */
        @kotlin.jvm.JvmName(name = "putPlayers")
        public final void putPlayers(@org.jetbrains.annotations.NotNull()
        com.google.protobuf.kotlin.DslMap<java.lang.String, java.lang.Long, com.tron.bridge.GetLeaderboardResponseKt.Dsl.PlayersProxy> $this$put, @org.jetbrains.annotations.NotNull()
        java.lang.String key, long value) {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/GetLeaderboardResponseKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/GetLeaderboardResponseKt$Dsl;", "builder", "Lcom/tron/bridge/Leaderboard$GetLeaderboardResponse$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
        
        /**
         * An uninstantiable, behaviorless type to represent the field in
         * generics.
         */
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0003"}, d2 = {"Lcom/tron/bridge/GetLeaderboardResponseKt$Dsl$PlayersProxy;", "Lcom/google/protobuf/kotlin/DslProxy;", "()V", "proxy"})
        @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
        public static final class PlayersProxy extends com.google.protobuf.kotlin.DslProxy {
            
            private PlayersProxy() {
                super();
            }
        }
    }
}