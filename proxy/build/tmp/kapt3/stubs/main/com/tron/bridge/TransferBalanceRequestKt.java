package com.tron.bridge;

/**
 * Protobuf type `bridge.TransferBalanceRequest`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/TransferBalanceRequestKt;", "", "()V", "Dsl", "proxy"})
public final class TransferBalanceRequestKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.TransferBalanceRequestKt INSTANCE = null;
    
    private TransferBalanceRequestKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00000\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\t\n\u0002\b\u0005\n\u0002\u0010\u000e\n\u0002\b\t\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0004\b\u0007\u0018\u0000 \u001b2\u00020\u0001:\u0001\u001bB\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0015\u001a\u00020\u0016H\u0001J\u0006\u0010\u0017\u001a\u00020\u0018J\u0006\u0010\u0019\u001a\u00020\u0018J\u0006\u0010\u001a\u001a\u00020\u0018R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011R$\u0010\u0012\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0013\u0010\u000f\"\u0004\b\u0014\u0010\u0011\u00a8\u0006\u001c"}, d2 = {"Lcom/tron/bridge/TransferBalanceRequestKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Balance$TransferBalanceRequest$Builder;", "(Lcom/tron/bridge/Balance$TransferBalanceRequest$Builder;)V", "value", "", "amount", "getAmount", "()J", "setAmount", "(J)V", "", "receiver", "getReceiver", "()Ljava/lang/String;", "setReceiver", "(Ljava/lang/String;)V", "sender", "getSender", "setSender", "_build", "Lcom/tron/bridge/Balance$TransferBalanceRequest;", "clearAmount", "", "clearReceiver", "clearSender", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Balance.TransferBalanceRequest.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.TransferBalanceRequestKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Balance.TransferBalanceRequest.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getSender")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getSender() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setSender")
        public final void setSender(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string sender = 1;`
         */
        public final void clearSender() {
        }
        
        @kotlin.jvm.JvmName(name = "getReceiver")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getReceiver() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setReceiver")
        public final void setReceiver(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string receiver = 2;`
         */
        public final void clearReceiver() {
        }
        
        @kotlin.jvm.JvmName(name = "getAmount")
        public final long getAmount() {
            return 0L;
        }
        
        @kotlin.jvm.JvmName(name = "setAmount")
        public final void setAmount(long value) {
        }
        
        /**
         * `uint64 amount = 3;`
         */
        public final void clearAmount() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/TransferBalanceRequestKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/TransferBalanceRequestKt$Dsl;", "builder", "Lcom/tron/bridge/Balance$TransferBalanceRequest$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}