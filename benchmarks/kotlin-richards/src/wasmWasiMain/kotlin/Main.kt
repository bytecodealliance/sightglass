import kotlin.wasm.WasmImport
import kotlin.wasm.unsafe.withScopedMemoryAllocator

// ===========================================================================
// Sightglass benchmarking hooks.
//
// The harness records execution time / performance counters between these two
// calls. As core-module imports they are `(import "bench" "start" (func))` and
// `(import "bench" "end" (func))`; after `wasm-tools component new` they surface
// as an imported `bench` instance exporting `start`/`end`, which is exactly what
// the sightglass component runner supplies.
// ===========================================================================

@WasmImport("bench", "start")
private external fun benchStart()

@WasmImport("bench", "end")
private external fun benchEnd()

// ===========================================================================
// Minimal WASI preview1 bindings for reading the workload file.
//
// Kotlin/Wasm's `wasmWasi` standard library has no general file API, so we bind
// the handful of `wasi_snapshot_preview1` functions we need directly. The
// WASI-preview1-to-component adapter lowers these to `wasi:filesystem` when the
// component is created.
// ===========================================================================

@WasmImport("wasi_snapshot_preview1", "path_open")
private external fun pathOpen(
    fd: Int,
    dirflags: Int,
    pathPtr: Int,
    pathLen: Int,
    oflags: Int,
    rightsBase: Long,
    rightsInheriting: Long,
    fdflags: Int,
    resultFdPtr: Int,
): Int

@WasmImport("wasi_snapshot_preview1", "fd_read")
private external fun fdRead(fd: Int, iovsPtr: Int, iovsLen: Int, nreadPtr: Int): Int

@WasmImport("wasi_snapshot_preview1", "fd_close")
private external fun fdClose(fd: Int): Int

/**
 * The benchmark's working directory is preopened by the harness as the first
 * (and only) WASI preopen, which is conventionally file descriptor 3.
 */
private const val PREOPEN_FD = 3

/** `__WASI_RIGHTS_FD_READ`: the right to `fd_read` the opened file. */
private const val RIGHTS_FD_READ = 2L

/**
 * Open [path] in the preopened working directory, read it, and return the
 * leading decimal integer it contains (the Richards iteration count).
 */
private fun readIterationCount(path: String): Int = withScopedMemoryAllocator { allocator ->
    // Write the path bytes into linear memory for `path_open`.
    val pathBytes = path.encodeToByteArray()
    val pathPtr = allocator.allocate(pathBytes.size)
    for (i in pathBytes.indices) {
        (pathPtr + i).storeByte(pathBytes[i])
    }

    // Open the file relative to the preopened directory.
    val fdResult = allocator.allocate(4)
    val openRc = pathOpen(
        PREOPEN_FD,
        0,
        pathPtr.address.toInt(),
        pathBytes.size,
        0,
        RIGHTS_FD_READ,
        0L,
        0,
        fdResult.address.toInt(),
    )
    check(openRc == 0) { "path_open(\"$path\") failed with errno $openRc" }
    val fd = fdResult.loadInt()

    // Read the file into a buffer via a single iovec { ptr, len }.
    val capacity = 64
    val buffer = allocator.allocate(capacity)
    val iovec = allocator.allocate(8)
    iovec.storeInt(buffer.address.toInt())
    (iovec + 4).storeInt(capacity)
    val nreadPtr = allocator.allocate(4)
    val readRc = fdRead(fd, iovec.address.toInt(), 1, nreadPtr.address.toInt())
    check(readRc == 0) { "fd_read failed with errno $readRc" }
    val bytesRead = nreadPtr.loadInt()
    fdClose(fd)

    // Parse the leading decimal integer out of the bytes read.
    var value = 0
    var sawDigit = false
    for (i in 0 until bytesRead) {
        val c = (buffer + i).loadByte().toInt()
        if (c in '0'.code..'9'.code) {
            value = value * 10 + (c - '0'.code)
            sawDigit = true
        } else if (sawDigit) {
            break
        }
    }
    check(sawDigit) { "no integer found in \"$path\"" }
    value
}

fun main() {
    // 1. Read the workload (the number of Richards iterations) from the
    //    filesystem via WASI.
    val iterations = readIterationCount("default.input")

    // 2. Begin recording.
    benchStart()

    // 3. The main work: run the Richards scheduler simulation `iterations`
    //    times, accumulating the packet-queue and task-hold counts each run
    //    produces.
    var totalQueueCount = 0L
    var totalHoldCount = 0L
    var allValid = true
    for (i in 0 until iterations) {
        val scheduler = runRichards()
        totalQueueCount += scheduler.queueCount
        totalHoldCount += scheduler.holdCount
        if (scheduler.queueCount != EXPECTED_QUEUE_COUNT ||
            scheduler.holdCount != EXPECTED_HOLD_COUNT
        ) {
            allValid = false
        }
    }

    // 4. Stop recording.
    benchEnd()

    // 5. Print results derived from the work so an optimizing compiler cannot
    //    elide the simulation.
    println("[kotlin-richards] iterations: $iterations")
    println("[kotlin-richards] queue count: $totalQueueCount")
    println("[kotlin-richards] hold count: $totalHoldCount")
    println("[kotlin-richards] " + if (allValid) "verified" else "INVALID")
}
