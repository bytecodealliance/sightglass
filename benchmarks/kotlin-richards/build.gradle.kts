@file:OptIn(org.jetbrains.kotlin.gradle.ExperimentalWasmDsl::class)

import org.jetbrains.kotlin.gradle.ExperimentalWasmDsl
import org.jetbrains.kotlin.gradle.tasks.KotlinCompilationTask

plugins {
    kotlin("multiplatform") version "2.3.0"
}

repositories {
    mavenCentral()
}

kotlin {
    wasmWasi {
        // A WASI "command": emits a core module exporting `_start` (whose body is
        // `fun main()`) plus linear `memory`, ready for the preview1->component
        // command adapter.
        nodejs()
        binaries.executable()
    }
}

tasks.withType<KotlinCompilationTask<*>>().configureEach {
    compilerOptions.freeCompilerArgs.addAll(
        // Opt in to the experimental host-import and unsafe-linear-memory APIs we
        // use for the `bench` hooks and WASI file reading.
        "-opt-in=kotlin.wasm.ExperimentalWasmInterop",
        "-opt-in=kotlin.wasm.unsafe.UnsafeWasmMemoryApi",
    )
}
