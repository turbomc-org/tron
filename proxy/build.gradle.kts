import com.google.protobuf.gradle.*
import org.jetbrains.gradle.ext.settings
import org.jetbrains.gradle.ext.taskTriggers

plugins {
    kotlin("jvm") version "2.0.20-Beta1"
    kotlin("kapt") version "2.0.20-Beta1"
    id("com.github.johnrengelman.shadow") version "8.1.1"
    id("eclipse")
    id("org.jetbrains.gradle.plugin.idea-ext") version "1.1.8"
    id("xyz.jpenilla.run-velocity") version "2.3.1"
    id("com.google.protobuf") version "0.9.4"
}

group = "com.h01.tron"

version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
    maven("https://repo.papermc.io/repository/maven-public/") { name = "papermc-repo" }
    maven("https://repo.opencollab.dev/main/") { name = "opencollab-repo" }
    maven {
        url = uri("https://repo.skyblocksquad.de/repo")
    }
}

dependencies {
    compileOnly("com.velocitypowered:velocity-api:3.4.0-SNAPSHOT")
    kapt("com.velocitypowered:velocity-api:3.4.0-SNAPSHOT")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3") // latest version
    implementation(
            "org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.7.3"
    ) // for JVM + blocking interop
    implementation("de.timongcraft:VeloBoard:1.5.4")

    // gRPC + Protobuf dependencies
    implementation("io.grpc:grpc-core:1.62.2")
    implementation("io.grpc:grpc-protobuf:1.62.2")
    implementation("io.grpc:grpc-stub:1.62.2")
    implementation("io.grpc:grpc-kotlin-stub:1.4.0")
    implementation("com.google.protobuf:protobuf-kotlin:3.25.3")
    implementation("com.google.protobuf:protobuf-java-util:3.25.3")
    implementation("io.grpc:grpc-okhttp:1.62.2")
    implementation("io.grpc:grpc-util:1.62.2")
    implementation("net.kyori:adventure-text-minimessage:4.25.0")

    compileOnly("org.geysermc.geyser:api:2.8.3-SNAPSHOT")
    compileOnly("org.geysermc.floodgate:api:2.2.4-SNAPSHOT")

    testImplementation("junit:junit:4.13.2")
}

tasks {
    runVelocity { velocityVersion("3.4.0-SNAPSHOT") }
    shadowJar {
        mergeServiceFiles() // Merges all META-INF/services files (e.g., for LoadBalancerProvider)
        archiveClassifier.set("") // Optional: Drops the "-all" suffix from the output JAR name
    }
}

// Protobuf / gRPC Kotlin configuration
protobuf {
    protoc { artifact = "com.google.protobuf:protoc:3.25.3" }

    plugins {
        // grpc-java plugin (for Java stubs)
        create("grpc") { artifact = "io.grpc:protoc-gen-grpc-java:1.62.2" }
        // grpc-kotlin plugin (for Kotlin stubs)
        create("grpckt") { artifact = "io.grpc:protoc-gen-grpc-kotlin:1.4.0:jdk8@jar" }
    }

    generateProtoTasks {
        all().forEach { task ->
            task.plugins {
                id("grpc") // Java stubs
                id("grpckt") // Kotlin stubs
            }
            task.builtins { id("kotlin") }
        }
    }

    sourceSets {
        getByName("main").proto {
            srcDir("../proto/") // your proto files
        }
    }
}

// Kotlin JVM toolchain
val targetJavaVersion = 17

kotlin { jvmToolchain(targetJavaVersion) }

// Template generation
val templateSource = file("src/main/templates")
val templateDest = layout.buildDirectory.dir("generated/sources/templates")
val generateTemplates =
        tasks.register<Copy>("generateTemplates") {
            val props = mapOf("version" to project.version)
            inputs.properties(props)

            from(templateSource)
            into(templateDest)
            expand(props)
        }

sourceSets.main.configure { java.srcDir(generateTemplates.map { it.outputs }) }

project.idea.project.settings.taskTriggers.afterSync(generateTemplates)

project.eclipse.synchronizationTasks(generateTemplates)
