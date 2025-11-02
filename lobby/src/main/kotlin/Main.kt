package com.h01.tron

import net.minestom.server.MinecraftServer

fun main() {
    val minecraftServer = MinecraftServer.init()

    minecraftServer.start("0.0.0.0", 25565)
    println("Server is up and running")
}