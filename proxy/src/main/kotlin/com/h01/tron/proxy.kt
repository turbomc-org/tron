package com.h01.tron

import com.google.inject.Inject
import com.h01.tron.commands.BalanceCommand
import com.h01.tron.commands.FriendsCommand
import com.h01.tron.commands.PayCommand
import com.h01.tron.commands.TeamsCommand
import com.h01.tron.events.SessionEvents
import com.h01.tron.leaderboard.LeaderboardManager
import com.tron.bridge.BridgeGrpcKt
import com.velocitypowered.api.command.CommandManager
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.plugin.Plugin
import com.velocitypowered.api.proxy.ProxyServer
import de.timongcraft.veloboard.VeloBoardRegistry
import io.grpc.ManagedChannel
import io.grpc.okhttp.OkHttpChannelBuilder
import org.slf4j.Logger

@Plugin(
    id = "proxy",
    name = "proxy",
    version = BuildConstants.VERSION,
    description = "Proxy plugin for the Minecraft network",
    url = "https://h01.in",
    authors = ["Harihar Nautiyal"]
)
class ProxyPlugin @Inject constructor(
    private val logger: Logger,
    private val server: ProxyServer
) {
    private lateinit var bridgeClient: BridgeGrpcKt.BridgeCoroutineStub
    private lateinit var sessionEvents: SessionEvents
    private lateinit var leaderboardManager: LeaderboardManager

    private lateinit var payCommand: PayCommand
    private lateinit var balanceCommand: BalanceCommand
    private lateinit var friendsCommand: FriendsCommand
    private lateinit var teamsCommand: TeamsCommand

    @Subscribe
    fun onProxyInitialization(event: ProxyInitializeEvent) {
        VeloBoardRegistry.register()

        val channel: ManagedChannel = OkHttpChannelBuilder
            .forAddress("127.0.0.1", 50051)
            .usePlaintext()
            .build()

        bridgeClient = BridgeGrpcKt.BridgeCoroutineStub(channel)
        sessionEvents = SessionEvents(bridgeClient, logger)
        leaderboardManager = LeaderboardManager(server, bridgeClient, logger)

        payCommand = PayCommand(bridgeClient, server)
        balanceCommand = BalanceCommand(bridgeClient)
        friendsCommand = FriendsCommand(bridgeClient, server)
        teamsCommand = TeamsCommand(bridgeClient, server)

        registerCommands()
        server.eventManager.register(this, sessionEvents)
        leaderboardManager.startRotation()

        logger.info("✅ Proxy initialized successfully with Leaderboards")
    }

    private fun registerCommands() {
        val manager: CommandManager = server.commandManager

        manager.register(
            manager.metaBuilder("pay").aliases("p", "transfer").plugin(this).build(),
            payCommand
        )

        manager.register(
            manager.metaBuilder("balance").aliases("bal", "b").plugin(this).build(),
            balanceCommand
        )

        manager.register(
            manager.metaBuilder("friend").aliases("friends", "f").plugin(this).build(),
            friendsCommand
        )

        manager.register(
            manager.metaBuilder("team").aliases("t").plugin(this).build(),
            teamsCommand
        )

        logger.info("Commands registered successfully")
    }
}
