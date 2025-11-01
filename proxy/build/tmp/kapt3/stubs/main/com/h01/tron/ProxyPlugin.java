package com.h01.tron;

@com.velocitypowered.api.plugin.Plugin(id = "proxy", name = "proxy", version = "1.0-SNAPSHOT", description = "Proxy plugin for the Minecraft network", url = "https://h01.in", authors = {"Harihar Nautiyal"})
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000P\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\b\u0007\u0018\u00002\u00020\u0001B\u0017\b\u0007\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J\u0010\u0010\u0015\u001a\u00020\u00162\u0006\u0010\u0017\u001a\u00020\u0018H\u0007J\b\u0010\u0019\u001a\u00020\u0016H\u0002R\u000e\u0010\u0007\u001a\u00020\bX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\t\u001a\u00020\nX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u000b\u001a\u00020\fX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\r\u001a\u00020\u000eX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u000f\u001a\u00020\u0010X\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0011\u001a\u00020\u0012X\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0013\u001a\u00020\u0014X\u0082.\u00a2\u0006\u0002\n\u0000\u00a8\u0006\u001a"}, d2 = {"Lcom/h01/tron/ProxyPlugin;", "", "logger", "Lorg/slf4j/Logger;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lorg/slf4j/Logger;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "balanceCommand", "Lcom/h01/tron/commands/BalanceCommand;", "bridgeClient", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "friendsCommand", "Lcom/h01/tron/commands/FriendsCommand;", "leaderboardManager", "Lcom/h01/tron/leaderboard/LeaderboardManager;", "payCommand", "Lcom/h01/tron/commands/PayCommand;", "sessionEvents", "Lcom/h01/tron/events/SessionEvents;", "teamsCommand", "Lcom/h01/tron/commands/TeamsCommand;", "onProxyInitialization", "", "event", "Lcom/velocitypowered/api/event/proxy/ProxyInitializeEvent;", "registerCommands", "proxy"})
public final class ProxyPlugin {
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    private com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient;
    private com.h01.tron.events.SessionEvents sessionEvents;
    private com.h01.tron.leaderboard.LeaderboardManager leaderboardManager;
    private com.h01.tron.commands.PayCommand payCommand;
    private com.h01.tron.commands.BalanceCommand balanceCommand;
    private com.h01.tron.commands.FriendsCommand friendsCommand;
    private com.h01.tron.commands.TeamsCommand teamsCommand;
    
    @com.google.inject.Inject()
    public ProxyPlugin(@org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger, @org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.proxy.ProxyServer server) {
        super();
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onProxyInitialization(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.proxy.ProxyInitializeEvent event) {
    }
    
    private final void registerCommands() {
    }
}