package com.tron.bridge;

/**
 * Protobuf type `bridge.SendMessageResponse`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/SendMessageResponseKt;", "", "()V", "Dsl", "proxy"})
public final class SendMessageResponseKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.SendMessageResponseKt INSTANCE = null;
    
    private SendMessageResponseKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00008\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\u000e\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\t\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0003\n\u0002\u0010\u000b\n\u0002\b\u0002\b\u0007\u0018\u0000 \u001d2\u00020\u0001:\u0001\u001dB\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0015\u001a\u00020\u0016H\u0001J\u0006\u0010\u0017\u001a\u00020\u0018J\u0006\u0010\u0019\u001a\u00020\u0018J\u0006\u0010\u001a\u001a\u00020\u0018J\u0006\u0010\u001b\u001a\u00020\u001cR\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011R$\u0010\u0012\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0013\u0010\t\"\u0004\b\u0014\u0010\u000b\u00a8\u0006\u001e"}, d2 = {"Lcom/tron/bridge/SendMessageResponseKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Messaging$SendMessageResponse$Builder;", "(Lcom/tron/bridge/Messaging$SendMessageResponse$Builder;)V", "value", "", "message", "getMessage", "()Ljava/lang/String;", "setMessage", "(Ljava/lang/String;)V", "Lcom/tron/bridge/Messaging$Prefix;", "prefix", "getPrefix", "()Lcom/tron/bridge/Messaging$Prefix;", "setPrefix", "(Lcom/tron/bridge/Messaging$Prefix;)V", "username", "getUsername", "setUsername", "_build", "Lcom/tron/bridge/Messaging$SendMessageResponse;", "clearMessage", "", "clearPrefix", "clearUsername", "hasPrefix", "", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Messaging.SendMessageResponse.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.SendMessageResponseKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Messaging.SendMessageResponse.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getPrefix")
        @org.jetbrains.annotations.NotNull()
        public final com.tron.bridge.Messaging.Prefix getPrefix() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setPrefix")
        public final void setPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Messaging.Prefix value) {
        }
        
        /**
         * `.bridge.Prefix prefix = 1;`
         */
        public final void clearPrefix() {
        }
        
        /**
         * `.bridge.Prefix prefix = 1;`
         * @return Whether the prefix field is set.
         */
        public final boolean hasPrefix() {
            return false;
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
         * `string username = 2;`
         */
        public final void clearUsername() {
        }
        
        @kotlin.jvm.JvmName(name = "getMessage")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getMessage() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setMessage")
        public final void setMessage(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string message = 3;`
         */
        public final void clearMessage() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/SendMessageResponseKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/SendMessageResponseKt$Dsl;", "builder", "Lcom/tron/bridge/Messaging$SendMessageResponse$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}