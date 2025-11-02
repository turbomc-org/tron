package com.h01.tron.listeners

import com.tron.bridge.BridgeGrpcKt
import com.tron.bridge.Server
import com.velocitypowered.api.proxy.ProxyServer
import io.grpc.StatusException
import io.grpc.StatusRuntimeException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import net.kyori.adventure.text.minimessage.MiniMessage
import org.slf4j.Logger
import java.io.IOException
import kotlin.coroutines.cancellation.CancellationException

class ServerMessageListener(
    private val bridgeClient: BridgeGrpcKt.BridgeCoroutineStub,
    private val server: ProxyServer,
    private val clientId: Long,
    private val logger: Logger
) {
    private val scope = CoroutineScope(Dispatchers.IO)
    private val mm = MiniMessage.miniMessage()
    @Volatile private var running = false


    fun startListening() {
        if (running) {
            logger.warn("⚠️ ServerMessageListener already running")
            return
        }
        running = true

        scope.launch {
            while (running) {
                try {
                    val request = Server.ServerSubscribeRequest.newBuilder()
                        .setClientId(clientId)
                        .build()

                    logger.info("🔌 Connecting to ServerSendMessage stream...")
                    val responseFlow = bridgeClient.serverSendMessage(request)

                    responseFlow.collect { response ->
                        val username = response.username
                        val message = response.message
                        val timestamp = response.timestamp

                        sendToPlayer(username, message)
                        logger.info("💬 [Server] $username -> $message ($timestamp)")
                    }

                    logger.warn("🔁 Stream ended normally. Reconnecting in 3s...")
                    delay(3000)

                } catch (e: CancellationException) {
                    logger.info("🛑 Listener cancelled: ${e.message}")
                    break
                } catch (e: StatusRuntimeException) {
                    logger.error("⚠️ gRPC connection lost: ${e.status}", e)
                    delay(5000)
                } catch (e: StatusException) {
                    logger.error("⚠️ gRPC stream exception: ${e.status}", e)
                    delay(5000)
                } catch (e: IOException) {
                    logger.error("⚠️ Network error: ${e.message}", e)
                    delay(5000)
                } catch (e: Exception) {
                    logger.error("❌ Unknown error in message listener: ${e.message}", e)
                    delay(5000)
                }
            }
        }
    }

    private fun sendToPlayer(username: String, message: String) {
        val playerOpt = server.getPlayer(username)

        if (playerOpt.isEmpty) {
            logger.warn("⚠️ Tried to send message to inactive/offline player: $username")
            return
        }

        val player = playerOpt.get()
        val parsedMessage = mm.deserialize(message)

        player.sendMessage(parsedMessage)
    }
}
