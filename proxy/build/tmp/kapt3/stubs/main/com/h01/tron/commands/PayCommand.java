package com.h01.tron.commands;

@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000L\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0011\n\u0002\u0010\u000e\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0002\b\u0002\n\u0002\u0010 \n\u0000\u0018\u00002\u00020\u0001B\u0015\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J\u0010\u0010\t\u001a\u00020\n2\u0006\u0010\u000b\u001a\u00020\fH\u0016J#\u0010\r\u001a\u00020\n2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u00120\u0011H\u0002\u00a2\u0006\u0002\u0010\u0013J\u0010\u0010\u0014\u001a\u00020\u00152\u0006\u0010\u000b\u001a\u00020\fH\u0016J\u0010\u0010\u0016\u001a\u00020\n2\u0006\u0010\u000e\u001a\u00020\u000fH\u0002J\u0016\u0010\u0017\u001a\b\u0012\u0004\u0012\u00020\u00120\u00182\u0006\u0010\u000b\u001a\u00020\fH\u0016R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0007\u001a\u00020\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000\u00a8\u0006\u0019"}, d2 = {"Lcom/h01/tron/commands/PayCommand;", "Lcom/velocitypowered/api/command/SimpleCommand;", "connection", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "scope", "Lkotlinx/coroutines/CoroutineScope;", "execute", "", "invocation", "Lcom/velocitypowered/api/command/SimpleCommand$Invocation;", "handleCommand", "player", "Lcom/velocitypowered/api/proxy/Player;", "args", "", "", "(Lcom/velocitypowered/api/proxy/Player;[Ljava/lang/String;)V", "hasPermission", "", "sendUsage", "suggest", "", "proxy"})
public final class PayCommand implements com.velocitypowered.api.command.SimpleCommand {
    @org.jetbrains.annotations.NotNull()
    private final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    @org.jetbrains.annotations.NotNull()
    private final kotlinx.coroutines.CoroutineScope scope = null;
    
    public PayCommand(@org.jetbrains.annotations.NotNull()
    com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection, @org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.proxy.ProxyServer server) {
        super();
    }
    
    @java.lang.Override()
    public void execute(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.command.SimpleCommand.Invocation invocation) {
    }
    
    private final void handleCommand(com.velocitypowered.api.proxy.Player player, java.lang.String[] args) {
    }
    
    private final void sendUsage(com.velocitypowered.api.proxy.Player player) {
    }
    
    @java.lang.Override()
    @org.jetbrains.annotations.NotNull()
    public java.util.List<java.lang.String> suggest(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.command.SimpleCommand.Invocation invocation) {
        return null;
    }
    
    @java.lang.Override()
    public boolean hasPermission(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.command.SimpleCommand.Invocation invocation) {
        return false;
    }
}