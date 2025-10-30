package com.tron.bridge;

/**
 * Protobuf type `bridge.CreatePrefixRequest`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/CreatePrefixRequestKt;", "", "()V", "Dsl", "proxy"})
public final class CreatePrefixRequestKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.CreatePrefixRequestKt INSTANCE = null;
    
    private CreatePrefixRequestKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00008\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\u000e\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0002\b\u0002\b\u0007\u0018\u0000 \u00192\u00020\u0001:\u0001\u0019B\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0012\u001a\u00020\u0013H\u0001J\u0006\u0010\u0014\u001a\u00020\u0015J\u0006\u0010\u0016\u001a\u00020\u0015J\u0006\u0010\u0017\u001a\u00020\u0018R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011\u00a8\u0006\u001a"}, d2 = {"Lcom/tron/bridge/CreatePrefixRequestKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Prefix$CreatePrefixRequest$Builder;", "(Lcom/tron/bridge/Prefix$CreatePrefixRequest$Builder;)V", "value", "Lcom/tron/bridge/Common$PartialPrefix;", "prefix", "getPrefix", "()Lcom/tron/bridge/Common$PartialPrefix;", "setPrefix", "(Lcom/tron/bridge/Common$PartialPrefix;)V", "", "username", "getUsername", "()Ljava/lang/String;", "setUsername", "(Ljava/lang/String;)V", "_build", "Lcom/tron/bridge/Prefix$CreatePrefixRequest;", "clearPrefix", "", "clearUsername", "hasPrefix", "", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Prefix.CreatePrefixRequest.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.CreatePrefixRequestKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Prefix.CreatePrefixRequest.Builder _builder) {
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
        
        @kotlin.jvm.JvmName(name = "getPrefix")
        @org.jetbrains.annotations.NotNull()
        public final com.tron.bridge.Common.PartialPrefix getPrefix() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setPrefix")
        public final void setPrefix(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Common.PartialPrefix value) {
        }
        
        /**
         * `.bridge.PartialPrefix prefix = 2;`
         */
        public final void clearPrefix() {
        }
        
        /**
         * `.bridge.PartialPrefix prefix = 2;`
         * @return Whether the prefix field is set.
         */
        public final boolean hasPrefix() {
            return false;
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/CreatePrefixRequestKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/CreatePrefixRequestKt$Dsl;", "builder", "Lcom/tron/bridge/Prefix$CreatePrefixRequest$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}