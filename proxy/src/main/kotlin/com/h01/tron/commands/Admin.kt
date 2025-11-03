package com.h01.tron.commands

import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Common
import com.tron.bridge.Prefix
import com.tron.bridge.Server
import com.tron.bridge.Teams
import com.velocitypowered.api.command.SimpleCommand
import com.velocitypowered.api.proxy.Player
import com.velocitypowered.api.proxy.ProxyServer
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.future
import kotlinx.coroutines.launch
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.format.NamedTextColor
import java.util.concurrent.CompletableFuture

class AdminCommand(
    private val connection: BridgeCoroutineStub,
    private val server: ProxyServer
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)
    private val subCommands = listOf("prefix", "team", "shop", "inc_coins")
    private val prefixCommands = listOf("create", "delete")
    private val teamCommands = listOf("delete")

    override fun execute(invocation: SimpleCommand.Invocation) {
        val source = invocation.source()
        val args = invocation.arguments()

        if (source !is Player) {
            source.sendMessage(Component.text("Only players can use this command", NamedTextColor.RED))
            return
        }

        handleCommand(source, args)
    }

    private fun handleCommand(player: Player, args: Array<String>) {
        scope.launch {
            if (args.isEmpty()) {
                sendUsage(player)
                return@launch
            }

            when (args[0].lowercase()) {
                "prefix" ->
                {
                    when (args[1].lowercase()) {
                        "create" -> {
                            if (args.size < 5) {
                                player.sendMessage(
                                    Component.text(
                                        "Usage: /prefix create <text> <color> <price>",
                                        NamedTextColor.YELLOW
                                    )
                                )
                                return@launch
                            }
                                val text = args[2]
                                val color = args[3]
                                val price: Long = args[4].trim().toLong()

                                createPrefix(player, text, color, price)
                        }
                        "delete" -> {
                            if (args.size < 3) {
                                player.sendMessage(Component.text("Usage: /prefix delete <name>", NamedTextColor.YELLOW))
                                return@launch
                            }

                            deletePrefix(player, args[2])
                        }
                    }
                }
                "team" -> {
                    when (args[1].lowercase()) {
                        "delete" -> {
                            if (args.size < 3) {
                                player.sendMessage(Component.text("Usage: /team delete <name>", NamedTextColor.YELLOW))
                                return@launch
                            }

                            deleteTeam(player, args[2])
                        }
                    }
                }
                "inc_coins" -> {
                    if (args.size < 3) {
                        player.sendMessage(
                            Component.text(
                                "Usage: /admin inc_coins <player> <coins>",
                                NamedTextColor.YELLOW
                            )
                        )
                        return@launch
                    }

                    increaseCoins(player, args[1], args[2].toLong())
                }
                else -> {
                    sendUsage(player)
                }
            }
        }
    }

    private fun sendUsage(player: Player) {
        player.sendMessage(
            Component.text("--- Friends Command ---", NamedTextColor.DARK_RED)
        )
        player.sendMessage(Component.text("/team delete <team>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/prefix add <name> <color> <price>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/prefix delete <name>", NamedTextColor.YELLOW))
    }

    private suspend fun deleteTeam(player: Player, teamName: String) {
        try {
            val request = Teams.DeleteTeamRequest.newBuilder().setUsername(player.username).setTeamName(teamName).build()
            connection.deleteTeam(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun increaseCoins(player: Player, target: String, amount: Long) {
        try {
            val request = Server.IncreaseCoinsRequest.newBuilder().setUsername(player.username).setTarget(target).setAmount(amount).build()
            connection.increaseCoins(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun deletePrefix(player: Player, prefixName: String) {
        try {
            val request = Prefix.DeletePrefixRequest.newBuilder().setUsername(player.username).setPrefix(prefixName).build()
            connection.deletePrefix(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun createPrefix(player: Player, text: String, color: String, price: Long) {
        try {
            val request = Prefix.CreatePrefixRequest.newBuilder().setUsername(player.username).setPrefix(Common.PartialPrefix.newBuilder().setText(text).setColor(color).setPrice(price).build()).build()
            connection.createPrefix(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    override fun suggestAsync(invocation: SimpleCommand.Invocation): CompletableFuture<List<String>> {
        val player = invocation.source() as? Player
            ?: return CompletableFuture.completedFuture(emptyList())
        val args = invocation.arguments()

        return scope.future {
            try {
                when (args.size) {
                    0 -> subCommands

                    1 -> subCommands.filter { it.startsWith(args[0], ignoreCase = true) }

                    2 -> {
                        when (args[0].lowercase()) {
                            "prefix" -> prefixCommands.filter { it.startsWith(args[1], ignoreCase = true) }
                            "team" -> teamCommands.filter { it.startsWith(args[1], ignoreCase = true) }
                            else -> emptyList()
                        }
                    }

                    3 -> {
                        val currentArg = args[2]
                        when (args[0].lowercase()) {
                            "team" -> when (args[1].lowercase()) {
                                "delete" -> {
                                    val request = Teams.GetAllTeamsRequest.getDefaultInstance()
                                    val response = connection.getAllTeams(request)
                                    response.teamsList
                                        .filter { it.startsWith(currentArg, ignoreCase = true) }
                                }
                                else -> emptyList()
                            }

                            "prefix" -> when (args[1].lowercase()) {
                                "delete" -> {
                                    val request = Prefix.GetAllPrefixRequest.getDefaultInstance()
                                    val response = connection.getAllPrefix(request)
                                    response.prefixesList
                                        .mapNotNull { it.text }
                                        .filter { it.startsWith(currentArg, ignoreCase = true) }
                                }
                                "create" -> {
                                    NamedTextColor.NAMES.keys()
                                        .filter { it.startsWith(currentArg, ignoreCase = true) }
                                }
                                else -> emptyList()
                            }

                            "inc_coins" -> {
                                server.allPlayers
                                    .map { it.username }
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                            }

                            else -> emptyList()
                        }
                    }

                    else -> emptyList()
                }
            } catch (e: Exception) {
                emptyList()
            }
        }
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation): Boolean = true
}