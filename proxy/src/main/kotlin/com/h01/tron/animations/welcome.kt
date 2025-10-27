package com.h01.tron.animations

import com.velocitypowered.api.proxy.Player
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import net.kyori.adventure.text.Component
import net.kyori.adventure.text.format.NamedTextColor
import net.kyori.adventure.title.Title
import java.time.Duration

public suspend fun welcome(player: Player, scope: CoroutineScope) {
    scope.launch {
        delay(100L)

        val mainTitleText = "Welcome to HALO MC"
        val subTitleText = "Get started by chatting with your AI assistant prometheus"
        val charDelay = 50L

        var currentMainTitle = ""
        for (char in mainTitleText) {
            currentMainTitle += char
            val mainTitle = Component.text(currentMainTitle, NamedTextColor.DARK_PURPLE)
            val subTitle = Component.text("") // Empty subtitle during main title animation
            val times = Title.Times.times(
                Duration.ofMillis(0),   // No fade-in for smoother typewriter effect
                Duration.ofMillis(100), // Short stay duration per character
                Duration.ofMillis(0)    // No fade-out
            )
            val title = Title.title(mainTitle, subTitle, times)
            player.showTitle(title)
            delay(charDelay)
        }

        var currentSubTitle = ""
        for (char in subTitleText) {
            currentSubTitle += char
            val mainTitle = Component.text(mainTitleText, NamedTextColor.DARK_PURPLE)
            val subTitle = Component.text(currentSubTitle, NamedTextColor.WHITE)
            val times = Title.Times.times(
                Duration.ofMillis(0),   // No fade-in
                Duration.ofMillis(100), // Short stay duration per character
                Duration.ofMillis(0)    // No fade-out
            )
            val title = Title.title(mainTitle, subTitle, times)
            player.showTitle(title)
            delay(charDelay)
        }

        val finalMainTitle = Component.text(mainTitleText, NamedTextColor.DARK_PURPLE)
        val finalSubTitle = Component.text(subTitleText, NamedTextColor.WHITE)
        val finalTimes = Title.Times.times(
            Duration.ofMillis(0),
            Duration.ofMillis(2000000000),
            Duration.ofMillis(500)
        )
        val finalTitle = Title.title(finalMainTitle, finalSubTitle, finalTimes)
        player.showTitle(finalTitle)
    }
}