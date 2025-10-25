package com.h01.tron;

import com.google.inject.Inject
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.plugin.Plugin
import org.slf4j.Logger
import com.tron.bridge.BridgeGrpcKt
import com.tron.bridge.Session
import com.tron.bridge.playerJoinRequest
import io.grpc.ManagedChannel
import kotlinx.coroutines.runBlocking
import io.grpc.okhttp.OkHttpChannelBuilder
import io.grpc.util.RoundRobinLoadBalancer

@Plugin(
    id = "proxy",
    name = "proxy",
    version = BuildConstants.VERSION,
    description = "Proxy plugin for the minecraft server",
    url = "https://h01.in",
    authors = ["Harihar Nautiyal"]
)
class proxy @Inject constructor(val logger: Logger) {
    lateinit var bridgeClient: BridgeGrpcKt.BridgeCoroutineStub
    lateinit var channel: ManagedChannel

    @Subscribe
    fun onProxyInitialization(event: ProxyInitializeEvent) {
        val channel: ManagedChannel = OkHttpChannelBuilder
            .forAddress("127.0.0.1", 50051)
            .usePlaintext()
            .build()
        bridgeClient = BridgeGrpcKt.BridgeCoroutineStub(channel)

        runBlocking {
            try {
                val request = playerJoinRequest {
                    username = "bootyqueen"
                    edition = Session.PlayerJoinRequest.Edition.JAVA
                }

                val response = bridgeClient.playerJoin(request)
                logger.info("PlayerJoin response: ${response.success}")
            } catch (e: Exception) {
                logger.error("Failed to call gRPC server", e)
            }
        }
    }
}
