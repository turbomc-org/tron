package com.h01.tron.listeners;

@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000H\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\t\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u000b\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\u000e\n\u0002\b\u0003\u0018\u00002\u00020\u0001B%\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u0012\u0006\u0010\u0006\u001a\u00020\u0007\u0012\u0006\u0010\b\u001a\u00020\t\u00a2\u0006\u0002\u0010\nJ\u0018\u0010\u0012\u001a\u00020\u00132\u0006\u0010\u0014\u001a\u00020\u00152\u0006\u0010\u0016\u001a\u00020\u0015H\u0002J\u0006\u0010\u0017\u001a\u00020\u0013R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0006\u001a\u00020\u0007X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\b\u001a\u00020\tX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u0013\u0010\u000b\u001a\u00070\f\u00a2\u0006\u0002\b\rX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u000e\u001a\u00020\u000fX\u0082\u000e\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0010\u001a\u00020\u0011X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000\u00a8\u0006\u0018"}, d2 = {"Lcom/h01/tron/listeners/ServerMessageListener;", "", "bridgeClient", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "clientId", "", "logger", "Lorg/slf4j/Logger;", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;Lcom/velocitypowered/api/proxy/ProxyServer;JLorg/slf4j/Logger;)V", "mm", "Lnet/kyori/adventure/text/minimessage/MiniMessage;", "Lorg/jetbrains/annotations/NotNull;", "running", "", "scope", "Lkotlinx/coroutines/CoroutineScope;", "sendToPlayer", "", "username", "", "message", "startListening", "proxy"})
public final class ServerMessageListener {
    @org.jetbrains.annotations.NotNull()
    private final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    private final long clientId = 0L;
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    @org.jetbrains.annotations.NotNull()
    private final kotlinx.coroutines.CoroutineScope scope = null;
    @org.jetbrains.annotations.NotNull()
    private final net.kyori.adventure.text.minimessage.MiniMessage mm = null;
    @kotlin.jvm.Volatile()
    private volatile boolean running = false;
    
    public ServerMessageListener(@org.jetbrains.annotations.NotNull()
    com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient, @org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.proxy.ProxyServer server, long clientId, @org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger) {
        super();
    }
    
    public final void startListening() {
    }
    
    private final void sendToPlayer(java.lang.String username, java.lang.String message) {
    }
}