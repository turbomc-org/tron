package com.tron.bridge;

/**
 * Protobuf type `bridge.ServerKickPlayerResponse`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/ServerKickPlayerResponseKt;", "", "()V", "Dsl", "proxy"})
public final class ServerKickPlayerResponseKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.ServerKickPlayerResponseKt INSTANCE = null;
    
    private ServerKickPlayerResponseKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00000\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\t\n\u0002\b\u0005\n\u0002\u0010\u000e\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0003\b\u0007\u0018\u0000 \u00172\u00020\u0001:\u0001\u0017B\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0012\u001a\u00020\u0013H\u0001J\u0006\u0010\u0014\u001a\u00020\u0015J\u0006\u0010\u0016\u001a\u00020\u0015R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011\u00a8\u0006\u0018"}, d2 = {"Lcom/tron/bridge/ServerKickPlayerResponseKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Server$ServerKickPlayerResponse$Builder;", "(Lcom/tron/bridge/Server$ServerKickPlayerResponse$Builder;)V", "value", "", "timestamp", "getTimestamp", "()J", "setTimestamp", "(J)V", "", "username", "getUsername", "()Ljava/lang/String;", "setUsername", "(Ljava/lang/String;)V", "_build", "Lcom/tron/bridge/Server$ServerKickPlayerResponse;", "clearTimestamp", "", "clearUsername", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Server.ServerKickPlayerResponse.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.ServerKickPlayerResponseKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Server.ServerKickPlayerResponse.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getUsername")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getUsername() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setUsername")
        public final void setUsername(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string username = 1;`
         */
        public final void clearUsername() {
        }
        
        @kotlin.jvm.JvmName(name = "getTimestamp")
        public final long getTimestamp() {
            return 0L;
        }
        
        @kotlin.jvm.JvmName(name = "setTimestamp")
        public final void setTimestamp(long value) {
        }
        
        /**
         * `uint64 timestamp = 2;`
         */
        public final void clearTimestamp() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/ServerKickPlayerResponseKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/ServerKickPlayerResponseKt$Dsl;", "builder", "Lcom/tron/bridge/Server$ServerKickPlayerResponse$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}