package com.h01.tron.events;

@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00006\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\u0018\u00002\u00020\u0001B\u0015\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J\u0010\u0010\t\u001a\u00020\n2\u0006\u0010\u000b\u001a\u00020\fH\u0007J\u0010\u0010\r\u001a\u00020\n2\u0006\u0010\u000b\u001a\u00020\u000eH\u0007J\u0010\u0010\u000f\u001a\u00020\n2\u0006\u0010\u000b\u001a\u00020\u0010H\u0007R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0007\u001a\u00020\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000\u00a8\u0006\u0011"}, d2 = {"Lcom/h01/tron/events/SessionEvents;", "", "connection", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "logger", "Lorg/slf4j/Logger;", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;Lorg/slf4j/Logger;)V", "scope", "Lkotlinx/coroutines/CoroutineScope;", "onDisconnect", "", "event", "Lcom/velocitypowered/api/event/connection/DisconnectEvent;", "onLogin", "Lcom/velocitypowered/api/event/connection/LoginEvent;", "onPostLogin", "Lcom/velocitypowered/api/event/connection/PostLoginEvent;", "proxy"})
public final class SessionEvents {
    @org.jetbrains.annotations.NotNull()
    private final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection = null;
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    @org.jetbrains.annotations.NotNull()
    private final kotlinx.coroutines.CoroutineScope scope = null;
    
    public SessionEvents(@org.jetbrains.annotations.NotNull()
    com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection, @org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger) {
        super();
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onPostLogin(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.connection.PostLoginEvent event) {
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onLogin(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.connection.LoginEvent event) {
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onDisconnect(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.connection.DisconnectEvent event) {
    }
}