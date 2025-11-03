package com.h01.tron;

@com.velocitypowered.api.plugin.Plugin(id = "proxy", name = "proxy", version = "1.0-SNAPSHOT", description = "Proxy plugin for the Minecraft network", url = "https://h01.in", authors = {"Harihar Nautiyal"})
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000^\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\t\n\u0002\b\b\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\b\u0007\u0018\u00002\u00020\u0001B\u0017\b\u0007\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J\u0010\u0010 \u001a\u00020!2\u0006\u0010\"\u001a\u00020#H\u0007J\b\u0010$\u001a\u00020!H\u0002R\u000e\u0010\u0007\u001a\u00020\bX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\t\u001a\u00020\nX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u000b\u001a\u00020\fX\u0082.\u00a2\u0006\u0002\n\u0000R+\u0010\u000f\u001a\u00020\u000e2\u0006\u0010\r\u001a\u00020\u000e8B@BX\u0082\u008e\u0002\u00a2\u0006\u0012\n\u0004\b\u0014\u0010\u0015\u001a\u0004\b\u0010\u0010\u0011\"\u0004\b\u0012\u0010\u0013R\u000e\u0010\u0016\u001a\u00020\u0017X\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0018\u001a\u00020\u0019X\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u001a\u001a\u00020\u001bX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u001c\u001a\u00020\u001dX\u0082.\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u001e\u001a\u00020\u001fX\u0082.\u00a2\u0006\u0002\n\u0000\u00a8\u0006%"}, d2 = {"Lcom/h01/tron/ProxyPlugin;", "", "logger", "Lorg/slf4j/Logger;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lorg/slf4j/Logger;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "adminCommand", "Lcom/h01/tron/commands/AdminCommand;", "balanceCommand", "Lcom/h01/tron/commands/BalanceCommand;", "bridgeClient", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "<set-?>", "", "clientId", "getClientId", "()J", "setClientId", "(J)V", "clientId$delegate", "Lkotlin/properties/ReadWriteProperty;", "friendsCommand", "Lcom/h01/tron/commands/FriendsCommand;", "payCommand", "Lcom/h01/tron/commands/PayCommand;", "prefixCommand", "Lcom/h01/tron/commands/PrefixCommand;", "sessionEvents", "Lcom/h01/tron/events/SessionEvents;", "teamsCommand", "Lcom/h01/tron/commands/TeamsCommand;", "onProxyInitialization", "", "event", "Lcom/velocitypowered/api/event/proxy/ProxyInitializeEvent;", "registerCommands", "proxy"})
public final class ProxyPlugin {
    @org.jetbrains.annotations.NotNull()
    private final org.slf4j.Logger logger = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    private com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub bridgeClient;
    private com.h01.tron.events.SessionEvents sessionEvents;
    private com.h01.tron.commands.PayCommand payCommand;
    private com.h01.tron.commands.BalanceCommand balanceCommand;
    private com.h01.tron.commands.FriendsCommand friendsCommand;
    private com.h01.tron.commands.TeamsCommand teamsCommand;
    private com.h01.tron.commands.PrefixCommand prefixCommand;
    private com.h01.tron.commands.AdminCommand adminCommand;
    @org.jetbrains.annotations.NotNull()
    private final kotlin.properties.ReadWriteProperty clientId$delegate = null;
    
    @com.google.inject.Inject()
    public ProxyPlugin(@org.jetbrains.annotations.NotNull()
    org.slf4j.Logger logger, @org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.proxy.ProxyServer server) {
        super();
    }
    
    private final long getClientId() {
        return 0L;
    }
    
    private final void setClientId(long p0) {
    }
    
    @com.velocitypowered.api.event.Subscribe()
    public final void onProxyInitialization(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.event.proxy.ProxyInitializeEvent event) {
    }
    
    private final void registerCommands() {
    }
}