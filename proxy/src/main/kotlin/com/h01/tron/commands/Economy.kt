package com.h01.tron.commands

import com.tron.bridge.Balance
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.velocitypowered.api.command.SimpleCommand
import com.velocitypowered.api.proxy.Player
import com.velocitypowered.api.proxy.ProxyServer
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.format.NamedTextColor
import net.kyori.adventure.text.format.TextDecoration
import org.geysermc.floodgate.api.FloodgateApi

class PayCommand(
    private val connection: BridgeCoroutineStub,
    private val server: ProxyServer,
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)

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
            if (args.size < 2) {
                sendUsage(player)
                return@launch
            }

            val receiverName = args[0]
            val amount = args[1].toLongOrNull() ?: 0
            if (amount <= 1) {
                player.sendMessage(Component.text("Amount must be greater than 1", NamedTextColor.RED))
                return@launch
            }

            val request = Balance.TransferBalanceRequest.newBuilder()
                .setSender(player.username)
                .setReceiver(receiverName)
                .setAmount(amount)
                .build()

            try {
                val response = connection.transferBalance(request)

                if (response.success) {
                    player.sendMessage(Component.text("\uD83D\uDCB8 $amount Hash Coins successfully transferred to $receiverName", NamedTextColor.GREEN))

                    val targetPlayer = server.getPlayer(receiverName).orElse(null)
                    if (targetPlayer != null) {
//                        val isBedrock = floodgateApi.isFloodgatePlayer(targetPlayer.uniqueId)
//                        if (isBedrock) {
//                            // You can show your GUI here if needed
//                        } else {
                            targetPlayer.sendMessage(
                                Component.text("${player.username} sent you $amount Hash Coins", NamedTextColor.YELLOW)
                            )
//                        }
                    }
                } else {
                    player.sendMessage(Component.text("Payment failed", NamedTextColor.RED))
                }

            } catch (e: Exception) {
                player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
            }
        }
    }

    private fun sendUsage(player: Player) {
        player.sendMessage(
            Component.text("--- Pay Command ---", NamedTextColor.DARK_RED)
        )
        player.sendMessage(Component.text("/pay <player> <amount>", NamedTextColor.YELLOW))
    }

    override fun suggest(invocation: SimpleCommand.Invocation): List<String> {
        val args = invocation.arguments()
        return if (args.size <= 1) {
            server.allPlayers.map { it.username }.filter { it.startsWith(args.getOrNull(0) ?: "", true) }
        } else emptyList()
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation) = true
}

class BalanceCommand(
    private val connection: BridgeCoroutineStub,
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)

    override fun execute(invocation: SimpleCommand.Invocation) {
        val source = invocation.source()

        if (source !is Player) {
            source.sendMessage(Component.text("Only players can use this command", NamedTextColor.RED))
            return
        }

        handleCommand(source)
    }

    private fun handleCommand(player: Player) {
        scope.launch {
            val request = Balance.GetBalanceRequest.newBuilder()
                .setUsername(player.username)
                .build()

            try {
                val response = connection.getBalance(request)

                player.sendMessage(
                    Component.text("Your balance is \uD83E\uDE99 ${response.balance} Hash Coins", NamedTextColor.YELLOW)
                )

            } catch (e: Exception) {
                player.sendMessage(
                    Component.text("${e.message}", NamedTextColor.RED)
                )
            }
        }
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation) = true
}
