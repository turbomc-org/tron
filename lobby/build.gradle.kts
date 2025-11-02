plugins {
    kotlin("jvm") version "2.3.20-dev-1363"
}

group = "com.h01.tron"
version = "1.0-SNAPSHOT"

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(25))
    }
}

kotlin {
    jvmToolchain(25)
}


repositories {
    mavenCentral()
    maven("https://redirector.kotlinlang.org/maven/bootstrap")
}

dependencies {
    implementation("net.minestom:minestom:2025.10.31-1.21.10")
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}


tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile>().configureEach {
    compilerOptions.jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_25)
}