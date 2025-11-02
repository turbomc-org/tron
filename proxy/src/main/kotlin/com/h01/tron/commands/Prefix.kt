package com.h01.tron.commands
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Friends
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
    private val subCommands = listOf("owned", "buy", "select", "current")

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
                "add" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /friend add <player>", NamedTextColor.YELLOW))
                        return@launch
                    }
                    sendFriendRequest(player, args[1])
                }
                "accept" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /friend accept <player>", NamedTextColor.YELLOW))
                        return@launch
                    }
                    acceptFriendRequest(player, args[1])
                }
                "reject" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /friend reject <player>", NamedTextColor.YELLOW))
                        return@launch
                    }
                    rejectFriendRequest(player, args[1])
                }
                "remove" -> {
                    if (args.size < 2) {
                        player.sendMessage(Component.text("Usage: /friend remove <player>", NamedTextColor.YELLOW))
                        return@launch
                    }
                    removeFriend(player, args[1])
                }
                "requests" -> listFriendRequests(player)
                "list" -> listFriends(player)
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

    private suspend fun listFriends(player: Player) {
        try {
            val request = Friends.GetFriendsRequest.newBuilder().setUsername(player.username).build()
            val response = connection.getFriends(request)
            if (response.friendsList.isEmpty()) {
                player.sendMessage(Component.text("You don't have any friends yet. Use /friend add <player> to add one!", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("--- Your Friends ---", NamedTextColor.GREEN))
                response.friendsList.forEach { friendName ->
                    val status = if (server.getPlayer(friendName).isPresent) "§aOnline" else "§cOffline"
                    player.sendMessage(Component.text("- $friendName ($status)", NamedTextColor.YELLOW))
                }
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun sendFriendRequest(player: Player, receiverName: String) {
        try {
            val request = Friends.SendFriendRequestRequest.newBuilder()
                .setSender(player.username)
                .setReceiver(receiverName)
                .build()

            val response = connection.sendFriendRequest(request)

            if (response.success) {
                player.sendMessage(Component.text("✅ Friend request sent to $receiverName", NamedTextColor.GREEN))
                server.getPlayer(receiverName).ifPresent {
                    it.sendMessage(Component.text("💌 ${player.username} sent you a friend request!", NamedTextColor.YELLOW))
                }
            } else {
                player.sendMessage(Component.text("Failed to send request. The player may not exist or has requests disabled.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun acceptFriendRequest(player: Player, senderName: String) {
        try {
            val request = Friends.AcceptFriendRequestRequest.newBuilder()
                .setUsername(player.username)
                .setSender(senderName)
                .build()
            val response = connection.acceptFriendRequest(request)
            if (response.success) {
                player.sendMessage(Component.text("🎉 You are now friends with $senderName!", NamedTextColor.GREEN))
                server.getPlayer(senderName).ifPresent {
                    it.sendMessage(Component.text("🎉 ${player.username} accepted your friend request!", NamedTextColor.GREEN))
                }
            } else {
                player.sendMessage(Component.text("Could not accept request. The request may not exist.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun rejectFriendRequest(player: Player, senderName: String) {
        try {
            val request = Friends.RejectFriendRequestRequest.newBuilder()
                .setUsername(player.username)
                .setSender(senderName)
                .build()
            val response = connection.rejectFriendRequest(request)
            if (response.success) {
                player.sendMessage(Component.text("You have rejected the friend request from $senderName", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("Could not reject request. The request may not exist.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun removeFriend(player: Player, targetName: String) {
        try {
            val request = Friends.RemoveFriendRequest.newBuilder()
                .setUsername(player.username)
                .setTarget(targetName)
                .build()
            val response = connection.removeFriend(request)
            if (response.success) {
                player.sendMessage(Component.text("🗑️ $targetName has been removed from your friends.", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("Failed to remove friend. Are you sure they are on your friends list?", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun listFriendRequests(player: Player) {
        try {
            val request = Friends.GetFriendRequestsRequest.newBuilder().setUsername(player.username).build()
            val response = connection.getFriendRequests(request)
            if (response.incomingFriendRequestsMap.isEmpty()) {
                player.sendMessage(Component.text("You have no pending friend requests.", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("--- Incoming Friend Requests ---", NamedTextColor.GREEN))
                response.incomingFriendRequestsMap.keys.forEach { sender ->
                    player.sendMessage(Component.text("- $sender", NamedTextColor.YELLOW))
                }
                player.sendMessage(Component.text("Use /friend accept <player> to accept.", NamedTextColor.GRAY))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
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
                            "add" -> {
                                server.allPlayers
                                    .map { it.username }
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                            }
                            "remove" -> {
                                val request = Friends.GetFriendsRequest.newBuilder().setUsername(player.username).build()
                                val response = connection.getFriends(request)
                                response.friendsList
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                            }
                            "accept", "reject" -> {
                                val request = Friends.GetFriendRequestsRequest.newBuilder().setUsername(player.username).build()
                                val response = connection.getFriendRequests(request)
                                response.incomingFriendRequestsMap.keys
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                            }
                            else -> emptyList()
                        }
                    }
                    else -> emptyList()
                }
            } catch (e: Exception) {
                // On error, return an empty list to prevent command failure
                emptyList<String>()
            }
        }
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation): Boolean = true
}