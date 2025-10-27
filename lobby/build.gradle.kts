plugins {
    kotlin("jvm") version "2.2.0"
}

group = "com.h01.tron"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}