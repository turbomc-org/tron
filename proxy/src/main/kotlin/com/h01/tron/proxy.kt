package com.h01.tron;

import com.google.inject.Inject
import com.h01.tron.commands.BalanceCommand
import com.h01.tron.commands.FriendsCommand
import com.h01.tron.commands.PayCommand
import com.h01.tron.commands.TeamsCommand
import com.h01.tron.events.SessionEvents
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.plugin.Plugin
import org.slf4j.Logger
import com.tron.bridge.BridgeGrpcKt
import com.velocitypowered.api.command.CommandManager
import com.velocitypowered.api.command.CommandMeta
import com.velocitypowered.api.proxy.ProxyServer
import io.grpc.ManagedChannel
import io.grpc.okhttp.OkHttpChannelBuilder
import jdk.jfr.Frequency

@Plugin(
    id = "proxy",
    name = "proxy",
    version = BuildConstants.VERSION,
    description = "Proxy plugin for the minecraft server",
    url = "https://h01.in",
    authors = ["Harihar Nautiyal"]
)
class proxy @Inject constructor(val logger: Logger, val server: ProxyServer) {
    lateinit var bridgeClient: BridgeGrpcKt.BridgeCoroutineStub
    lateinit var channel: ManagedChannel
    lateinit var sessionEvents: SessionEvents

    // Commands
    lateinit var payCommand: PayCommand
    lateinit var balanceCommand: BalanceCommand
    lateinit var friendsCommand: FriendsCommand
    lateinit var teamsCommand: TeamsCommand


    @Subscribe
    fun onProxyInitialization(event: ProxyInitializeEvent) {
        val channel: ManagedChannel = OkHttpChannelBuilder
            .forAddress("127.0.0.1", 50051)
            .usePlaintext()
            .build()
        bridgeClient = BridgeGrpcKt.BridgeCoroutineStub(channel)

        sessionEvents = SessionEvents(bridgeClient, logger)
        balanceCommand = BalanceCommand(bridgeClient)
        payCommand = PayCommand(bridgeClient, server)
        friendsCommand = FriendsCommand(bridgeClient, server)
        teamsCommand = TeamsCommand(bridgeClient, server)

        registerCommands()

        logger.info("Registering SessionEvents listener...")
        server.eventManager.register(this, sessionEvents)
        logger.info("Registered SessionEvents!")
    }


    private fun registerCommands() {
        val commandManager: CommandManager = server.commandManager

        val payCommandMeta: CommandMeta =
            commandManager.metaBuilder("transfer").aliases("p", "pay").plugin(this).build()

        val balanceCommandMeta: CommandMeta =
            commandManager.metaBuilder("balance").aliases("b", "bal").plugin(this).build()

        val friendsCommandMeta: CommandMeta =
            commandManager.metaBuilder("friend")
                .aliases("friends", "f")
                .plugin(this)
                .build()

        val teamsCommandMeta: CommandMeta =
            commandManager.metaBuilder("team")
                .aliases("t")
                .plugin(this)
                .build()

        commandManager.register(payCommandMeta, payCommand)
        commandManager.register(balanceCommandMeta, balanceCommand)
        commandManager.register(friendsCommandMeta, friendsCommand)
        commandManager.register(teamsCommandMeta, teamsCommand)

        logger.info("Commands registered")
    }
}
