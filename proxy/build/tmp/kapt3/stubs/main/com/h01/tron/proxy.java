package com.h01.tron;

@com.velocitypowered.api.plugin.Plugin(id = "proxy", name = "proxy", version = "1.0-SNAPSHOT", description = "Proxy plugin for the minecraft server", url = "https://h01.in", authors = {"Harihar Nautiyal"})
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000.\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\u0007\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0007\u0018\u00002\u00020\u0001B\u000f\b\u0007\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\u0010\u0010\u0013\u001a\u00020\u00142\u0006\u0010\u0015\u001a\u00020\u0016H\u0007R\u001a\u0010\u0005\u001a\u00020\u0006X\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\u0007\u0010\b\"\u0004\b\t\u0010\nR\u001a\u0010\u000b\u001a\u00020\fX\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\r\u0010\u000e\"\u0004\b\u000f\u0010\u0010R\u0011\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\b\n\u0000\u001a\u0004\b\u0011\u0010\u0012\u00a8\u0006\u0017"}, d2 = {"Lcom/h01/tron/proxy;", "", "logger", "Lorg/slf4j/Logger;", "(Lorg/slf4j/Logger;)V", "bridgeClient", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "getBridgeClient", "()Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "setBridgeClient", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;)V", "channel", "Lio/grpc/ManagedChannel;", "getChannel", "()Lio/grpc/ManagedChannel;", "setChannel", "(Lio/grpc/ManagedChannel;)V", "getLogger", "()Lorg/slf4j/Logger;", "onProxyInitialization", "", "event", "Lcom/velocitypowered/api/event/proxy/ProxyInitializeEvent;", "proxy"})
public final class proxy {
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    public com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient;
    public io.grpc.ManagedChannel channel;
    
    @com.google.inject.Inject()
    public proxy(@org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger) {
        super();
    }
    
    @org.jetbrains.annotations.NotNull()
    public final org.slf4j.Logger getLogger() {
        return null;
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub getBridgeClient() {
        return null;
    }
    
    public final void setBridgeClient(@org.jetbrains.annotations.NotNull()
    com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub p0) {
    }
    
    @org.jetbrains.annotations.NotNull()
    public final io.grpc.ManagedChannel getChannel() {
        return null;
    }
    
    public final void setChannel(@org.jetbrains.annotations.NotNull()
    io.grpc.ManagedChannel p0) {
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onProxyInitialization(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.proxy.ProxyInitializeEvent event) {
    }
}