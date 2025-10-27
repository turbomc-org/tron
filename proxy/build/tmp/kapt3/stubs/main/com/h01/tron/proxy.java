package com.h01.tron;

@com.velocitypowered.api.plugin.Plugin(id = "proxy", name = "proxy", version = "1.0-SNAPSHOT", description = "Proxy plugin for the minecraft server", url = "https://h01.in", authors = {"Harihar Nautiyal"})
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000^\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\u0007\n\u0002\u0018\u0002\n\u0002\b\u0007\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\b\u0007\u0018\u00002\u00020\u0001B\u0017\b\u0007\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J\u0010\u00105\u001a\u0002062\u0006\u00107\u001a\u000208H\u0007J\b\u00109\u001a\u000206H\u0002R\u001a\u0010\u0007\u001a\u00020\bX\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\t\u0010\n\"\u0004\b\u000b\u0010\fR\u001a\u0010\r\u001a\u00020\u000eX\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\u000f\u0010\u0010\"\u0004\b\u0011\u0010\u0012R\u001a\u0010\u0013\u001a\u00020\u0014X\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\u0015\u0010\u0016\"\u0004\b\u0017\u0010\u0018R\u001a\u0010\u0019\u001a\u00020\u001aX\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b\u001b\u0010\u001c\"\u0004\b\u001d\u0010\u001eR\u0011\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\b\n\u0000\u001a\u0004\b\u001f\u0010 R\u001a\u0010!\u001a\u00020\"X\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b#\u0010$\"\u0004\b%\u0010&R\u0011\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\b\n\u0000\u001a\u0004\b\'\u0010(R\u001a\u0010)\u001a\u00020*X\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b+\u0010,\"\u0004\b-\u0010.R\u001a\u0010/\u001a\u000200X\u0086.\u00a2\u0006\u000e\n\u0000\u001a\u0004\b1\u00102\"\u0004\b3\u00104\u00a8\u0006:"}, d2 = {"Lcom/h01/tron/proxy;", "", "logger", "Lorg/slf4j/Logger;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lorg/slf4j/Logger;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "balanceCommand", "Lcom/h01/tron/commands/BalanceCommand;", "getBalanceCommand", "()Lcom/h01/tron/commands/BalanceCommand;", "setBalanceCommand", "(Lcom/h01/tron/commands/BalanceCommand;)V", "bridgeClient", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "getBridgeClient", "()Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "setBridgeClient", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;)V", "channel", "Lio/grpc/ManagedChannel;", "getChannel", "()Lio/grpc/ManagedChannel;", "setChannel", "(Lio/grpc/ManagedChannel;)V", "friendsCommand", "Lcom/h01/tron/commands/FriendsCommand;", "getFriendsCommand", "()Lcom/h01/tron/commands/FriendsCommand;", "setFriendsCommand", "(Lcom/h01/tron/commands/FriendsCommand;)V", "getLogger", "()Lorg/slf4j/Logger;", "payCommand", "Lcom/h01/tron/commands/PayCommand;", "getPayCommand", "()Lcom/h01/tron/commands/PayCommand;", "setPayCommand", "(Lcom/h01/tron/commands/PayCommand;)V", "getServer", "()Lcom/velocitypowered/api/proxy/ProxyServer;", "sessionEvents", "Lcom/h01/tron/events/SessionEvents;", "getSessionEvents", "()Lcom/h01/tron/events/SessionEvents;", "setSessionEvents", "(Lcom/h01/tron/events/SessionEvents;)V", "teamsCommand", "Lcom/h01/tron/commands/TeamsCommand;", "getTeamsCommand", "()Lcom/h01/tron/commands/TeamsCommand;", "setTeamsCommand", "(Lcom/h01/tron/commands/TeamsCommand;)V", "onProxyInitialization", "", "event", "Lcom/velocitypowered/api/event/proxy/ProxyInitializeEvent;", "registerCommands", "proxy"})
public final class proxy {
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    public com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient;
    public io.grpc.ManagedChannel channel;
    public com.h01.tron.events.SessionEvents sessionEvents;
    public com.h01.tron.commands.PayCommand payCommand;
    public com.h01.tron.commands.BalanceCommand balanceCommand;
    public com.h01.tron.commands.FriendsCommand friendsCommand;
    public com.h01.tron.commands.TeamsCommand teamsCommand;
    
    @com.google.inject.Inject()
    public proxy(@org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger, @org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.proxy.ProxyServer server) {
        super();
    }
    
    @org.jetbrains.annotations.NotNull()
    public final org.slf4j.Logger getLogger() {
        return null;
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.velocitypowered.api.proxy.ProxyServer getServer() {
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
    
    @org.jetbrains.annotations.NotNull()
    public final com.h01.tron.events.SessionEvents getSessionEvents() {
        return null;
    }
    
    public final void setSessionEvents(@org.jetbrains.annotations.NotNull()
    com.h01.tron.events.SessionEvents p0) {
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.h01.tron.commands.PayCommand getPayCommand() {
        return null;
    }
    
    public final void setPayCommand(@org.jetbrains.annotations.NotNull()
    com.h01.tron.commands.PayCommand p0) {
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.h01.tron.commands.BalanceCommand getBalanceCommand() {
        return null;
    }
    
    public final void setBalanceCommand(@org.jetbrains.annotations.NotNull()
    com.h01.tron.commands.BalanceCommand p0) {
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.h01.tron.commands.FriendsCommand getFriendsCommand() {
        return null;
    }
    
    public final void setFriendsCommand(@org.jetbrains.annotations.NotNull()
    com.h01.tron.commands.FriendsCommand p0) {
    }
    
    @org.jetbrains.annotations.NotNull()
    public final com.h01.tron.commands.TeamsCommand getTeamsCommand() {
        return null;
    }
    
    public final void setTeamsCommand(@org.jetbrains.annotations.NotNull()
    com.h01.tron.commands.TeamsCommand p0) {
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onProxyInitialization(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.proxy.ProxyInitializeEvent event) {
    }
    
    private final void registerCommands() {
    }
}