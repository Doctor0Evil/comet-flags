plugins {
    kotlin("jvm") version "2.0.0"
    application
}

group = "ai.comet.flags"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

application {
    mainClass.set("ai.comet.flags.AllianceProtocolsKt")
}

tasks.test {
    useJUnitPlatform()
}
