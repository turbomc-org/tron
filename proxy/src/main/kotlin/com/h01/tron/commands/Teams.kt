package com.h01.tron.commands

import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Teams
import com.tron.bridge.getOpenTeamsRequest
import com.velocitypowered.api.command.SimpleCommand
import com.velocitypowered.api.proxy.Player
import com.velocitypowered.api.proxy.ProxyServer
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.future
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.format.NamedTextColor
import java.util.concurrent.CompletableFuture

class TeamsCommand(
    private val connection: BridgeCoroutineStub,
    private val server: ProxyServer,
) : SimpleCommand {

    private val scope = CoroutineScope(Dispatchers.IO)
    private val subCommands = listOf(
        "create", "leave", "join", "invite", "accept", "reject",
        "members", "promote", "open", "delete"
    )

    override fun execute(invocation: SimpleCommand.Invocation) {
        val source = invocation.source()
        val args = invocation.arguments()

        if (source !is Player) {
            source.sendMessage(Component.text("Only players can use this command.", NamedTextColor.RED))
            return
        }

        scope.launch { handleCommand(source, args) }
    }

    private suspend fun handleCommand(player: Player, args: Array<String>) {
        if (args.isEmpty()) {
            sendUsage(player)
            return
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
            "open" -> listOpenTeams(player)
            "delete" -> deleteTeam(player, args)
            else -> sendUsage(player)
        }
    }

    private fun sendUsage(player: Player) {
        player.sendMessage(Component.text("--- Teams Command ---", NamedTextColor.DARK_PURPLE))
        player.sendMessage(Component.text("/team create <name> <color> [open|closed]", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team invite <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team join <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team accept <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team reject <name>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team promote <player>", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team members", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team leave", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team open", NamedTextColor.YELLOW))
        player.sendMessage(Component.text("/team delete <name>", NamedTextColor.YELLOW))
    }

    private suspend fun createTeam(player: Player, args: Array<String>) {
        if (args.size < 3) {
            player.sendMessage(
                Component.text(
                    "Usage: /team create <name> <color> [open|closed]",
                    NamedTextColor.YELLOW
                )
            )
            return
        }
        val teamName = args[1]
        val color = args[2]
        val isOpen = if (args.size > 3) args[3].equals("open", ignoreCase = true) else true

        try {
            val request = Teams.CreateTeamRequest.newBuilder()
                .setUsername(player.username)
                .setTeam(teamName)
                .setColor(color)
                .setOpen(isOpen)
                .build()
            connection.createTeam(request)
        } catch (e: Exception) {
            player.sendMessage(Component.text("Error: ${e.message}", NamedTextColor.RED))
        }
    }

    private suspend fun leaveTeam(player: Player) {
        try {
            connection.leaveTeam(
                Teams.LeaveTeamRequest.newBuilder().setUsername(player.username).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun joinTeam(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team join <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        try {
            connection.joinTeam(
                Teams.JoinTeamRequest.newBuilder().setUsername(player.username).setTeam(teamName).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun sendTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team invite <player>", NamedTextColor.YELLOW))
            return
        }
        val targetName = args[1]
        try {
            connection.sendTeamInvite(
                Teams.SendTeamInviteRequest.newBuilder().setUsername(player.username).setTarget(targetName).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun acceptTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team accept <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        try {
            connection.acceptTeamInvite(
                Teams.AcceptTeamInviteRequest.newBuilder().setUsername(player.username).setTeam(teamName).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun rejectTeamInvite(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team reject <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        try {
           connection.rejectTeamInvite(
                Teams.RejectTeamInviteRequest.newBuilder().setUsername(player.username).setTeam(teamName).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun getTeamMembers(player: Player) {
        try {
            connection.getTeamMembers(
                Teams.GetTeamMembersRequest.newBuilder().setUsername(player.username).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun promoteTeamMember(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team promote <player>", NamedTextColor.YELLOW))
            return
        }
        val target = args[1]
        try {
            connection.promoteTeamMember(
                Teams.PromoteTeamMemberRequest.newBuilder().setUsername(player.username).setTarget(target).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun listOpenTeams(player: Player) {
        try {
            connection.getOpenTeams(Teams.GetOpenTeamsRequest.getDefaultInstance())
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    private suspend fun deleteTeam(player: Player, args: Array<String>) {
        if (args.size < 2) {
            player.sendMessage(Component.text("Usage: /team delete <name>", NamedTextColor.YELLOW))
            return
        }
        val teamName = args[1]
        try {
            connection.deleteTeam(
                Teams.DeleteTeamRequest.newBuilder().setTeamName(teamName).build()
            )
        } catch (e: Exception) {
            player.sendMessage(Component.text(e.message.toString(), NamedTextColor.RED))
        }
    }

    override fun suggestAsync(invocation: SimpleCommand.Invocation): CompletableFuture<List<String>> {
        val player = invocation.source() as? Player ?: return CompletableFuture.completedFuture(emptyList())
        val args = invocation.arguments()

        return scope.future {
            try {
                when {
                    // ✅ No subcommand typed yet
                    args.size <= 1 -> {
                        subCommands.filter { it.startsWith(args.getOrNull(0) ?: "", ignoreCase = true) }
                    }

                    // ✅ Suggest based on subcommand
                    args.size == 2 -> {
                        val currentArg = args[1]
                        when (args[0].lowercase()) {

                            // /team join <name>  → list open teams
                            "join" -> {
                                val res = connection.getOpenTeams(Teams.GetOpenTeamsRequest.getDefaultInstance())
                                res.teamsList.filter { it.startsWith(currentArg, ignoreCase = true) }
                            }

                            // /team create <name> → placeholder
                            "create" -> listOf("<team_name>").filter {
                                it.startsWith(currentArg, ignoreCase = true)
                            }

                            // /team delete <name> → open teams or owned team (for now open teams)
                            "delete" -> {
                                val res = connection.getOpenTeams(Teams.GetOpenTeamsRequest.getDefaultInstance())
                                res.teamsList.filter { it.startsWith(currentArg, ignoreCase = true) }
                            }

                            // /team invite <player> → list all online players not in your team
                            "invite" -> {
                                val membersResponse = connection.getTeamMembers(
                                    Teams.GetTeamMembersRequest.newBuilder()
                                        .setUsername(player.username)
                                        .build()
                                )

                                val teamMembers = membersResponse.membersList.toSet()
                                server.allPlayers
                                    .map { it.username }
                                    .filter { it.startsWith(currentArg, ignoreCase = true) }
                                    .filterNot { it in teamMembers || it == player.username }
                            }

                            // /team promote <player> → team members except self
                            "promote" -> {
                                val res = connection.getTeamMembers(
                                    Teams.GetTeamMembersRequest.newBuilder()
                                        .setUsername(player.username)
                                        .build()
                                )
                                res.membersList
                                    .filter { it != player.username && it.startsWith(currentArg, ignoreCase = true) }
                            }

                            // /team accept <team> → open teams (for now, later could come from invites)
                            "accept", "reject" -> {
                                val res = connection.getOpenTeams(Teams.GetOpenTeamsRequest.getDefaultInstance())
                                res.teamsList.filter { it.startsWith(currentArg, ignoreCase = true) }
                            }

                            // fallback: filter subcommands
                            else -> subCommands.filter { it.startsWith(args[0], ignoreCase = true) }
                        }
                    }

                    // ✅ Handle 3rd arg, e.g., /team create <name> <color>
                    args.size == 3 && args[0].equals("create", ignoreCase = true) -> {
                        val currentArg = args[2]
                        NamedTextColor.NAMES.keys()
                            .filter { it.startsWith(currentArg, ignoreCase = true) }
                            .toList()
                    }

                    // ✅ Handle 4th arg for /team create (open|closed)
                    args.size == 4 && args[0].equals("create", ignoreCase = true) -> {
                        listOf("open", "closed").filter {
                            it.startsWith(args[3], ignoreCase = true)
                        }
                    }

                    else -> emptyList()
                }
            } catch (e: Exception) {
                emptyList<String>()
            }
        }
    }


    override fun hasPermission(invocation: SimpleCommand.Invocation) = true
}
