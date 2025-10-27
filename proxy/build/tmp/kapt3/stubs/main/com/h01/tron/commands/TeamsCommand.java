package com.h01.tron.commands;

@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000L\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010 \n\u0002\u0010\u000e\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0011\n\u0002\b\u0004\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\u000b\n\u0002\b\b\u0018\u00002\u00020\u0001B\u0015\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u0012\u0006\u0010\u0004\u001a\u00020\u0005\u00a2\u0006\u0002\u0010\u0006J$\u0010\f\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J$\u0010\u0013\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J\u0010\u0010\u0014\u001a\u00020\r2\u0006\u0010\u0015\u001a\u00020\u0016H\u0016J\u0016\u0010\u0017\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000fH\u0082@\u00a2\u0006\u0002\u0010\u0018J#\u0010\u0019\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0002\u00a2\u0006\u0002\u0010\u001aJ\u0010\u0010\u001b\u001a\u00020\u001c2\u0006\u0010\u0015\u001a\u00020\u0016H\u0016J$\u0010\u001d\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J\u0016\u0010\u001e\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000fH\u0082@\u00a2\u0006\u0002\u0010\u0018J$\u0010\u001f\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J$\u0010 \u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J$\u0010!\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000f2\f\u0010\u0010\u001a\b\u0012\u0004\u0012\u00020\u000b0\u0011H\u0082@\u00a2\u0006\u0002\u0010\u0012J\u0010\u0010\"\u001a\u00020\r2\u0006\u0010\u000e\u001a\u00020\u000fH\u0002J\u0016\u0010#\u001a\b\u0012\u0004\u0012\u00020\u000b0\n2\u0006\u0010\u0015\u001a\u00020\u0016H\u0016R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0007\u001a\u00020\bX\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u000e\u0010\u0004\u001a\u00020\u0005X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u0014\u0010\t\u001a\b\u0012\u0004\u0012\u00020\u000b0\nX\u0082\u0004\u00a2\u0006\u0002\n\u0000\u00a8\u0006$"}, d2 = {"Lcom/h01/tron/commands/TeamsCommand;", "Lcom/velocitypowered/api/command/SimpleCommand;", "connection", "Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;", "server", "Lcom/velocitypowered/api/proxy/ProxyServer;", "(Lcom/tron/bridge/BridgeGrpcKt$BridgeCoroutineStub;Lcom/velocitypowered/api/proxy/ProxyServer;)V", "scope", "Lkotlinx/coroutines/CoroutineScope;", "subCommands", "", "", "acceptTeamInvite", "", "player", "Lcom/velocitypowered/api/proxy/Player;", "args", "", "(Lcom/velocitypowered/api/proxy/Player;[Ljava/lang/String;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "createTeam", "execute", "invocation", "Lcom/velocitypowered/api/command/SimpleCommand$Invocation;", "getTeamMembers", "(Lcom/velocitypowered/api/proxy/Player;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "handleCommand", "(Lcom/velocitypowered/api/proxy/Player;[Ljava/lang/String;)V", "hasPermission", "", "joinTeam", "leaveTeam", "promoteTeamMember", "rejectTeamInvite", "sendTeamInvite", "sendUsage", "suggest", "proxy"})
public final class TeamsCommand implements com.velocitypowered.api.command.SimpleCommand {
    @org.jetbrains.annotations.NotNull()
    private final com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub connection = null;
    @org.jetbrains.annotations.NotNull()
    private final com.velocitypowered.api.proxy.ProxyServer server = null;
    @org.jetbrains.annotations.NotNull()
    private final kotlinx.coroutines.CoroutineScope scope = null;
    @org.jetbrains.annotations.NotNull()
    private final java.util.List<java.lang.String> subCommands = null;
    
    public TeamsCommand(@org.jetbrains.annotations.NotNull()
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
    
    private final java.lang.Object createTeam(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object leaveTeam(com.velocitypowered.api.proxy.Player player, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object joinTeam(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object sendTeamInvite(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object acceptTeamInvite(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object rejectTeamInvite(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object getTeamMembers(com.velocitypowered.api.proxy.Player player, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
    }
    
    private final java.lang.Object promoteTeamMember(com.velocitypowered.api.proxy.Player player, java.lang.String[] args, kotlin.coroutines.Continuation<? super kotlin.Unit> $completion) {
        return null;
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