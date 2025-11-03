package com.h01.tron.commands
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Friends
import com.tron.bridge.Prefix
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

class PrefixCommand(
    private val connection: BridgeCoroutineStub,
    private val server: ProxyServer
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)
    private val subCommands = listOf("owned", "buy", "equip", "current", "list", "unequip")

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
                "owned" ->
                    listOwnedPrefixes(player)
                "list" ->
                    listAllPrefixes(player)
                "current" -> {
                    getCurrentPrefix(player)
                }
                "buy" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /prefix buy <prefix>", NamedTextColor.YELLOW))
                        return@launch
                    }

                    buyPrefix(player, args[1])
                }
                "equip" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /prefix equip <prefix>", NamedTextColor.YELLOW))
                        return@launch
                    }

                    equipPrefix(player, args[1])
                }
                "unequip" -> unequipPrefix(player)
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
        player.sendMessage(Component.text("/friend add <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/friend remove <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/friend accept <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/friend reject <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/friend requests", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/friend list", NamedTextColor.YELLOW))
    }

    private suspend fun getCurrentPrefix(player: Player) {
        try {
            val request = Prefix.GetCurrentPrefixRequest.newBuilder().setUsername(player.username).build()
            connection.getCurrentPrefix(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun listAllPrefixes(player: Player) {
        try {
            val request = Prefix.ListAllPrefixRequest.newBuilder().setUsername(player.username).build()
            connection.listAllPrefix(request)
        } catch(e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun listOwnedPrefixes(player: Player) {
        try {
            val request = Prefix.ListOwnedPrefixRequest.newBuilder().setUsername(player.username).build()
            connection.listOwnedPrefix(request)
        } catch(e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun buyPrefix(player: Player, prefixName: String) {
        try {
            val request = Prefix.BuyPrefixRequest.newBuilder().setUsername(player.username).setPrefix(prefixName).build()
            connection.buyPrefix(request)
        } catch(e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun equipPrefix(player: Player, prefixName: String) {
        try {
           val request = Prefix.EquipPrefixRequest.newBuilder().setUsername(player.username).setPrefix(prefixName).build()
           connection.equipPrefix(request)
        } catch(e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun unequipPrefix(player: Player) {
        try {
            val request = Prefix.UnEquipPrefixRequest.newBuilder().setUsername(player.username).build()
            connection.unEquipPrefix(request)
        } catch(e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    override fun suggestAsync(invocation: SimpleCommand.Invocation): CompletableFuture<List<String>> {
        val player = invocation.source() as? Player ?: return CompletableFuture.completedFuture(emptyList())
        val args = invocation.arguments()

        return scope.future {
            try {
                when {
                    args.size <= 1 -> {
                        subCommands.filter { it.startsWith(args.getOrNull(0) ?: "", ignoreCase = true) }
                    }
                    args.size == 2 -> {
                        val currentArg = args[1]
                        when (args[0].lowercase()) {
                            "buy" -> {
                                val request = Prefix.GetAllPrefixRequest.getDefaultInstance()
                                val response = connection.getAllPrefix(request)
                                response.prefixesList.filter {it.text.startsWith(currentArg, ignoreCase = true)}.map { prefix -> prefix.text }
                            }
                            "equip", "unequip" -> {
                                val request = Prefix.GetOwnedPrefixRequest.newBuilder().setUsername(player.username).build()
                                val response = connection.getOwnedPrefix(request)
                                response.prefixesList
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                            }
                            else -> emptyList()
                        }
                    }
                    else -> emptyList()
                }
            } catch (e: Exception) {
                emptyList<String>()
            }
        }
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation): Boolean = true
}