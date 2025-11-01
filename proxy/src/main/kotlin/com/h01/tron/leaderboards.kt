package com.h01.tron

import com.tron.bridge.*
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.velocitypowered.api.proxy.ProxyServer
import de.timongcraft.veloboard.VeloBoard
import kotlinx.coroutines.*
import net.kyori.adventure.text.Component
import org.slf4j.Logger
import java.util.*

class LeaderboardManager(
    private val server: ProxyServer,
    private val connection: BridgeCoroutineStub,
    private val logger: Logger
) {
    private val boards: MutableMap<UUID, VeloBoard> = HashMap()
    private val coroutineScope = CoroutineScope(Dispatchers.IO)
    private var currentTypeIndex = 0

    private val leaderboardTypes = listOf(
        "Kills", "Deaths", "Coins", "KDR", "Teams", "Overall"
    )

    fun startRotation() {
        coroutineScope.launch {
            while (true) {
                try {
                    val type = leaderboardTypes[currentTypeIndex]
                    val leaderboard = fetchLeaderboard(type)

                    for (player in server.allPlayers) {
                        val board = boards.computeIfAbsent(player.uniqueId) {
                            VeloBoard(player)
                        }
                        updateBoard(board, type, leaderboard)
                    }

                    currentTypeIndex = (currentTypeIndex + 1) % leaderboardTypes.size
                } catch (e: Exception) {
                    logger.error("Leaderboard rotation error: ${e.message}", e)
                }

                delay(5000L) // rotate every 5 seconds
            }
        }
    }

    private suspend fun fetchLeaderboard(type: String): Map<String, String> {
        return when (type) {
            "Kills" -> connection.killsLeaderboard(Leaderboard.KillsLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { it.value.toString() }

            "Deaths" -> connection.deathsLeaderboard(Leaderboard.DeathsLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { it.value.toString() }

            "Coins" -> connection.coinsLeaderboard(Leaderboard.CoinsLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { it.value.toString() }

            "KDR" -> connection.kdaLeaderboard(Leaderboard.KdaLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { "%.2f".format(it.value) }

            "Teams" -> connection.teamsLeaderboard(Leaderboard.TeamsLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { it.value.toString() }

            else -> connection.overallLeaderboard(Leaderboard.OverallLeaderboardRequest.newBuilder().build())
                .leaderboardMap.mapValues { it.value.toString() }
        }
    }

    private fun updateBoard(board: VeloBoard, type: String, leaderboard: Map<String, String>) {
        val lines = mutableListOf<Component>()
        lines.add(Component.text("§6§l===== $type Leaderboard ====="))
        var rank = 1
        leaderboard.entries
            .sortedByDescending { it.value.toDoubleOrNull() ?: 0.0 }
            .take(10)
            .forEach { (name, value) ->
                lines.add(Component.text("§e#$rank §f$name §7- §b$value"))
                rank++
            }

        while (lines.size < 12) lines.add(Component.empty())
        board.updateLines(*lines.toTypedArray())
    }
}
