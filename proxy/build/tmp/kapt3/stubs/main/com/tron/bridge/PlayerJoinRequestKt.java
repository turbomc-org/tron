package com.tron.bridge;

/**
 * Protobuf type `bridge.PlayerJoinRequest`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/PlayerJoinRequestKt;", "", "()V", "Dsl", "proxy"})
public final class PlayerJoinRequestKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.PlayerJoinRequestKt INSTANCE = null;
    
    private PlayerJoinRequestKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00008\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\b\n\u0002\b\u0005\n\u0002\u0010\u000e\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0003\b\u0007\u0018\u0000 \u001d2\u00020\u0001:\u0001\u001dB\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0018\u001a\u00020\u0019H\u0001J\u0006\u0010\u001a\u001a\u00020\u001bJ\u0006\u0010\u001c\u001a\u00020\u001bR\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011R$\u0010\u0013\u001a\u00020\u00122\u0006\u0010\u0005\u001a\u00020\u00128G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0014\u0010\u0015\"\u0004\b\u0016\u0010\u0017\u00a8\u0006\u001e"}, d2 = {"Lcom/tron/bridge/PlayerJoinRequestKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Session$PlayerJoinRequest$Builder;", "(Lcom/tron/bridge/Session$PlayerJoinRequest$Builder;)V", "value", "Lcom/tron/bridge/Session$PlayerJoinRequest$Edition;", "edition", "getEdition", "()Lcom/tron/bridge/Session$PlayerJoinRequest$Edition;", "setEdition", "(Lcom/tron/bridge/Session$PlayerJoinRequest$Edition;)V", "", "editionValue", "getEditionValue", "()I", "setEditionValue", "(I)V", "", "username", "getUsername", "()Ljava/lang/String;", "setUsername", "(Ljava/lang/String;)V", "_build", "Lcom/tron/bridge/Session$PlayerJoinRequest;", "clearEdition", "", "clearUsername", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Session.PlayerJoinRequest.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.PlayerJoinRequestKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Session.PlayerJoinRequest.Builder _builder) {
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
        
        @kotlin.jvm.JvmName(name = "getEdition")
        @org.jetbrains.annotations.NotNull()
        public final com.tron.bridge.Session.PlayerJoinRequest.Edition getEdition() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setEdition")
        public final void setEdition(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Session.PlayerJoinRequest.Edition value) {
        }
        
        @kotlin.jvm.JvmName(name = "getEditionValue")
        public final int getEditionValue() {
            return 0;
        }
        
        @kotlin.jvm.JvmName(name = "setEditionValue")
        public final void setEditionValue(int value) {
        }
        
        /**
         * `.bridge.PlayerJoinRequest.Edition edition = 2;`
         */
        public final void clearEdition() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/PlayerJoinRequestKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/PlayerJoinRequestKt$Dsl;", "builder", "Lcom/tron/bridge/Session$PlayerJoinRequest$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}