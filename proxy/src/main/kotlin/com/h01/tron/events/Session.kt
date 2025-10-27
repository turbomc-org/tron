package com.h01.tron.events

import com.google.inject.util.Types.listOf
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub
import com.tron.bridge.Session
import com.tron.bridge.playerJoinRequest
import com.tron.bridge.playerLeaveRequest
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.DisconnectEvent
import com.velocitypowered.api.event.connection.LoginEvent
import com.velocitypowered.api.proxy.Player
import net.kyori.adventure.dialog.DialogLike
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.event.ClickEvent
import net.kyori.adventure.text.format.NamedTextColor
import net.kyori.adventure.title.Title
import org.slf4j.Logger
import kotlin.system.measureTimeMillis
import com.h01.tron.animations.welcome
import com.velocitypowered.api.event.connection.PostLoginEvent
import kotlinx.coroutines.delay

class SessionEvents(private val connection: BridgeCoroutineStub, private val logger: Logger) {
    private val scope = CoroutineScope(Dispatchers.IO)

    @Subscribe
    fun onPostLogin(event: PostLoginEvent) {
        val player = event.player;
        scope.launch {
            welcome(player, scope)
        }
    }

    @Subscribe
    fun onLogin(event: LoginEvent) {
        val player: Player = event.player

        logger.info("Sending join request to main server")

        scope.launch {
            val request = playerJoinRequest {
                username = player.username
                edition = Session.PlayerJoinRequest.Edition.JAVA
            }

            try {
                val duration = measureTimeMillis {
                    val response = connection.playerJoin(request)
                    logger.info("Join response for ${player.username}: success=${response.success}")

                    if (!response.success) {
                        player.disconnect(Component.text("Failed to verify your account from main server"))
                    }
                }
                logger.info("Join request for ${player.username} completed in ${duration}ms")
            } catch (e: Exception) {
                logger.error("Failed to send join request", e)
                player.disconnect(Component.text("Failed to verify your account from main server"))
            }
        }
    }

    @Subscribe
    fun onDisconnect(event: DisconnectEvent) {
        val player: Player = event.player

        logger.info("Sending disconnection request to main server")

        scope.launch {
            val request = playerLeaveRequest { username = player.username }

            try {
                val duration = measureTimeMillis {
                    connection.playerLeave(request)
                }
                logger.info("Leave request for ${player.username} completed in ${duration}ms")
            } catch (e: Exception) {
                logger.error("Failed to send leave request", e)
            }
        }
    }
}
