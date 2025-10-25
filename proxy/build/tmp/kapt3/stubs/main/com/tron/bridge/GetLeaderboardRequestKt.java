package com.tron.bridge;

/**
 * Protobuf type `bridge.GetLeaderboardRequest`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/GetLeaderboardRequestKt;", "", "()V", "Dsl", "proxy"})
public final class GetLeaderboardRequestKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.GetLeaderboardRequestKt INSTANCE = null;
    
    private GetLeaderboardRequestKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00000\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\b\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0002\b\u0007\u0018\u0000 \u00162\u00020\u0001:\u0001\u0016B\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0012\u001a\u00020\u0013H\u0001J\u0006\u0010\u0014\u001a\u00020\u0015R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011\u00a8\u0006\u0017"}, d2 = {"Lcom/tron/bridge/GetLeaderboardRequestKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$Builder;", "(Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$Builder;)V", "value", "Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$SortBy;", "sorting", "getSorting", "()Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$SortBy;", "setSorting", "(Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$SortBy;)V", "", "sortingValue", "getSortingValue", "()I", "setSortingValue", "(I)V", "_build", "Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest;", "clearSorting", "", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Leaderboard.GetLeaderboardRequest.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.GetLeaderboardRequestKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Leaderboard.GetLeaderboardRequest.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getSorting")
        @org.jetbrains.annotations.NotNull()
        public final com.tron.bridge.Leaderboard.GetLeaderboardRequest.SortBy getSorting() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setSorting")
        public final void setSorting(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Leaderboard.GetLeaderboardRequest.SortBy value) {
        }
        
        @kotlin.jvm.JvmName(name = "getSortingValue")
        public final int getSortingValue() {
            return 0;
        }
        
        @kotlin.jvm.JvmName(name = "setSortingValue")
        public final void setSortingValue(int value) {
        }
        
        /**
         * `.bridge.GetLeaderboardRequest.SortBy sorting = 1;`
         */
        public final void clearSorting() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/GetLeaderboardRequestKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/GetLeaderboardRequestKt$Dsl;", "builder", "Lcom/tron/bridge/Leaderboard$GetLeaderboardRequest$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}