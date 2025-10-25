package com.h01.tron.events

import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.LoginEvent
import com.velocitypowered.api.event.connection.DisconnectEvent
import com.velocitypowered.api.proxy.Player
import com.tron.bridge.BridgeGrpcKt.BridgeCoroutineStub;
import com.tron.bridge.Session
import com.tron.bridge.playerJoinRequest
import com.tron.bridge.playerLeaveRequest

class SessionEvents(private val connection: BridgeCoroutineStub) {
    @Subscribe
    suspend fun onPlayerJoin(event: LoginEvent) {
        val player: Player = event.player

        val request = playerJoinRequest {
            username = player.username
            edition = Session.PlayerJoinRequest.Edition.JAVA
        }

        val response = connection.playerJoin(request)

        if (!response.success) {
            TODO("KICK THE PLAYER")
        }
    }

    @Subscribe
    suspend fun onPlayerLeft(event: DisconnectEvent) {
        val player: Player = event.player

        val request = playerLeaveRequest {
            username = player.username
        }

        val response = connecti
    }
}