package com.h01.tron.commands;

@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000^\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010 \n\u0002\u0010\u000e\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010\t\n\u0002\b\b\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\u0011\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\u0018\u00002\u00020\u0001B\u0015\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J.\u0010\u000e\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u0012\u001a\u00020\t2\u0006\u0010\u0013\u001a\u00020\t2\u0006\u0010\u0014\u001a\u00020\u0015H\u0082@\u00a2\u0006\u0002\u0010\u0016J\u001e\u0010\u0017\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u0018\u001a\u00020\tH\u0082@\u00a2\u0006\u0002\u0010\u0019J\u001e\u0010\u001a\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u001b\u001a\u00020\tH\u0082@\u00a2\u0006\u0002\u0010\u0019J\u0010\u0010\u001c\u001a\u00020\u000f2\u0006\u0010\u001d\u001a\u00020\u001eH\u0016J#\u0010\u001f\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u00112\f\u0010 \u001a\b\u0012\u0004\u0012\u00020\t0!H\u0002\u00a2\u0006\u0002\u0010\"J\u0010\u0010#\u001a\u00020$2\u0006\u0010\u001d\u001a\u00020\u001eH\u0016J&\u0010%\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u00112\u0006\u0010&\u001a\u00020\t2\u0006\u0010\'\u001a\u00020\u0015H\u0082@\u00a2\u0006\u0002\u0010(J\u0010\u0010)\u001a\u00020\u000f2\u0006\u0010\u0010\u001a\u00020\u0011H\u0002J\u001c\u0010*\u001a\u000e\u0012\n\u0012\b\u0012\u0004\u0012\u00020\t0\b0+2\u0006\u0010\u001d\u001a\u00020\u001eH\u0016R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u0014\u0010\u0007\u001a\b\u0012\u0004\u0012\u00020\t0\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\n\u001a\u00020\u000bX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u0014\u0010\f\u001a\b\u0012\u0004\u0012\u00020\t0\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u0014\u0010\r\u001a\b\u0012\u0004\u0012\u00020\t0\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000\u00a8\u0006,"}, d2 = {"Lcom/h01/tron/commands/AdminCommand;", "Lcom/velocitypowered/api/command/SimpleCommand;", "connection", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "prefixCommands", "", "", "scope", "Lkotlinx/coroutines/CoroutineScope;", "subCommands", "teamCommands", "createPrefix", "", "player", "Lcom/velocitypowered/api/proxy/Player;", "text", "color", "price", "", "(Lcom/velocitypowered/api/proxy/Player;Ljava/lang/String;Ljava/lang/String;JLkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deletePrefix", "prefixName", "(Lcom/velocitypowered/api/proxy/Player;Ljava/lang/String;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "deleteTeam", "teamName", "execute", "invocation", "Lcom/velocitypowered/api/command/SimpleCommand$Invocation;", "handleCommand", "args", "", "(Lcom/velocitypowered/api/proxy/Player;[Ljava/lang/String;)V", "hasPermission", "", "increaseCoins", "target", "amount", "(Lcom/velocitypowered/api/proxy/Player;Ljava/lang/String;JLkotlin/coroutines/Continuation;)Ljava/lang/Object;", "sendUsage", "suggestAsync", "Ljava/util/concurrent/CompletableFuture;", "proxy"})
public final class AdminCommand implements com.velocitypowered.api.command.SimpleCommand {
    @org.jetbrains.annotations.NotNull()
    private final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    @org.jetbrains.annotations.NotNull()
    private final kotlinx.coroutines.CoroutineScope scope = null;
    @org.jetbrains.annotations.NotNull()
    private final java.util.List<java.lang.String> subCommands = null;
    @org.jetbrains.annotations.NotNull()
    private final java.util.List<java.lang.String> prefixCommands = null;
    @org.jetbrains.annotations.NotNull()
    private final java.util.List<java.lang.String> teamCommands = null;
    
    public AdminCommand(@org.jetbrains.annotations.NotNull()
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
    
    private final java.lang.Object deleteTeam(com.velocitypowered.api.proxy.Player player, java.lang.String teamName, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object increaseCoins(com.velocitypowered.api.proxy.Player player, java.lang.String target, long amount, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object deletePrefix(com.velocitypowered.api.proxy.Player player, java.lang.String prefixName, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object createPrefix(com.velocitypowered.api.proxy.Player player, java.lang.String text, java.lang.String color, long price, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    @java.lang.Override()
    @org.jetbrains.annotations.NotNull()
    public java.util.concurrent.CompletableFuture<java.util.List<java.lang.String>> suggestAsync(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.command.SimpleCommand.Invocation invocation) {
        return null;
    }
    
    @java.lang.Override()
    public boolean hasPermission(@org.jetbrains.annotations.NotNull()
    com.velocitypowered.api.command.SimpleCommand.Invocation invocation) {
        return false;
    }
}