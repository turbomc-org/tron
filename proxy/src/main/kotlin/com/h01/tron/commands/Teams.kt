package com.h01.tron.commands

import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Teams
import com.velocitypowered.api.command.SimpleCommand
import com.velocitypowered.api.proxy.Player
import com.velocitypowered.api.proxy.ProxyServer
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.format.NamedTextColor
import net.kyori.adventure.text.format.TextDecoration

class TeamsCommand(
    private val connection: BridgeCoroutineStub,
    private val server: ProxyServer,
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)
    private val subCommands = listOf("create", "leave", "join", "invite", "accept", "reject", "members", "promote")

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
                "create" -> createTeam(player, args)
                "leave" -> leaveTeam(player)
                "join" -> joinTeam(player, args)
                "invite" -> sendTeamInvite(player, args)
                "accept" -> acceptTeamInvite(player, args)
                "reject" -> rejectTeamInvite(player, args)
                "members" -> getTeamMembers(player)
                "promote" -> promoteTeamMember(player, args)
                else -> sendUsage(player)
            }
        }
    }

    private fun sendUsage(player: Player) {
        player.sendMessage(Component.text("--- Teams Command ---", NamedTextColor.DARK_RED))
        player.sendMessage(Component.text("/team create <name> <color> [open|closed]", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team invite <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team join <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team accept <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team reject <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team promote <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team members", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team leave", NamedTextColor.YELLOW))
    }

    private suspend fun createTeam(player: Player, args: Array<String>) {
        if (args.size < 3) {
            player.sendMessage(Component.text("Usage: /team create <name> <color> [open|closed]", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        val color = args[2]
        val isOpen = if (args.size > 3) args[3].equals("open", ignoreCase = true) else true

        val request = Teams.CreateTeamRequest.newBuilder()
            .setUsername(player.username)
            .setTeam(teamName)
            .setColor(color)
            .setOpen(isOpen)
            .build()
        try {
            val response = connection.createTeam(request)
            if (response.success) {
                player.sendMessage(Component.text("🎉 Team '$teamName' successfully created!", NamedTextColor.GREEN))
            } else {
                player.sendMessage(Component.text("Failed to create team. You might already be in one or the name is taken.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun leaveTeam(player: Player) {
        val request = Teams.LeaveTeamRequest.newBuilder().setUsername(player.username).build()
        try {
            val response = connection.leaveTeam(request)
            if (response.success) {
                player.sendMessage(Component.text("You have successfully left your team.", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("Failed to leave team. Are you in one?", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun joinTeam(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team join <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        val request = Teams.JoinTeamRequest.newBuilder()
            .setUsername(player.username)
            .setTeam(teamName)
            .build()
        try {
            val response = connection.joinTeam(request)
            if (response.success) {
                player.sendMessage(Component.text("✅ You have joined team '$teamName'!", NamedTextColor.GREEN))
            } else {
                player.sendMessage(Component.text("Could not join team. It might be closed, full, or non-existent.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun sendTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team invite <player>", NamedTextColor.YELLOW))
            return
        }
        val targetName = args[1]
        val request = Teams.SendTeamInviteRequest.newBuilder()
            .setUsername(player.username)
            .setTarget(targetName)
            .build()
        try {
            val response = connection.sendTeamInvite(request)
            if (response.success) {
                player.sendMessage(Component.text("✅ Invite sent to $targetName.", NamedTextColor.GREEN))
                server.getPlayer(targetName).ifPresent { targetPlayer ->
                    targetPlayer.sendMessage(
                        Component.text("💌 ${player.username} has invited you to their team! Ask them for the team name and use /team accept <name> to join.", NamedTextColor.YELLOW)
                    )
                }
            } else {
                player.sendMessage(Component.text("Could not send invite. You may not be a team leader or the player is already in a team.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun acceptTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team accept <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        val request = Teams.AcceptTeamInviteRequest.newBuilder()
            .setUsername(player.username)
            .setTeam(teamName)
            .build()
        try {
            val response = connection.acceptTeamInvite(request)
            if (response.success) {
                player.sendMessage(Component.text("🎉 You have successfully joined team '$teamName'!", NamedTextColor.GREEN))
            } else {
                player.sendMessage(Component.text("Failed to accept invite. It may have expired or you are already in a team.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun rejectTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team reject <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        val request = Teams.RejectTeamInviteRequest.newBuilder()
            .setUsername(player.username)
            .setTeam(teamName)
            .build()
        try {
            val response = connection.rejectTeamInvite(request)
            if (response.success) {
                player.sendMessage(Component.text("You have rejected the invite from team '$teamName'.", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("Could not reject invite. Perhaps it was already rescinded.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun getTeamMembers(player: Player) {
        val request = Teams.GetTeamMembersRequest.newBuilder().setUsername(player.username).build()
        try {
            val response = connection.getTeamMembers(request)
            if (response.membersList.isEmpty()) {
                player.sendMessage(Component.text("You are not in a team. Use /team create or /team join!", NamedTextColor.YELLOW))
            } else {
                player.sendMessage(Component.text("--- Team Members ---", NamedTextColor.GREEN))
                response.membersList.forEach { memberName ->
                    val status = if (server.getPlayer(memberName).isPresent) "§aOnline" else "§cOffline"
                    player.sendMessage(Component.text("- $memberName ($status)", NamedTextColor.YELLOW))
                }
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun promoteTeamMember(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team promote <player>", NamedTextColor.YELLOW))
            return
        }
        val targetName = args[1]
        val request = Teams.PromoteTeamMemberRequest.newBuilder()
            .setUsername(player.username)
            .setTarget(targetName)
            .build()
        try {
            val response = connection.promoteTeamMember(request)
            if (response.success) {
                player.sendMessage(Component.text("✅ $targetName has been promoted in the team.", NamedTextColor.GREEN))
            } else {
                player.sendMessage(Component.text("Promotion failed. You must be the team leader.", NamedTextColor.RED))
            }
        } catch (e: Exception) {
            player.sendMessage(Component.text("${e.message}", NamedTextColor.RED))
        }
    }

    override fun suggest(invocation: SimpleCommand.Invocation): List<String> {
        val args = invocation.arguments()

        if (args.size <= 1) {
            return subCommands.filter { it.startsWith(args.getOrNull(0) ?: "", ignoreCase = true) }
        }

        if (args.size == 2) {
            when (args[0].lowercase()) {
                "invite", "promote" -> {
                    return server.allPlayers.map { it.username }.filter { it.startsWith(args[1], ignoreCase = true) }
                }
            }
        }

        return emptyList()
    }

    override fun hasPermission(invocation: SimpleCommand.Invocation) = true
}