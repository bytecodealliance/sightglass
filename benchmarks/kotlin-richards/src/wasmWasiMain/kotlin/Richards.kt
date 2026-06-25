/*
 * Richards: a faithful Kotlin port of the JetStream2 Octane `richards.js`
 * benchmark, which simulates the task dispatcher of an operating system.
 *
 * The original JavaScript (and this port's structure, constants, and the
 * EXPECTED_QUEUE_COUNT / EXPECTED_HOLD_COUNT validation values) come from the V8
 * project's Richards benchmark; see README.md for full provenance and license.
 */

const val ID_IDLE = 0
const val ID_WORKER = 1
const val ID_HANDLER_A = 2
const val ID_HANDLER_B = 3
const val ID_DEVICE_A = 4
const val ID_DEVICE_B = 5
const val NUMBER_OF_IDS = 6

const val KIND_DEVICE = 0
const val KIND_WORK = 1

const val STATE_RUNNING = 0
const val STATE_RUNNABLE = 1
const val STATE_SUSPENDED = 2
const val STATE_HELD = 4
const val STATE_SUSPENDED_RUNNABLE = STATE_SUSPENDED or STATE_RUNNABLE // 3
const val STATE_NOT_HELD = STATE_HELD.inv()                            // ~4

const val DATA_SIZE = 4
const val COUNT = 1000

/**
 * These two constants are characteristic of a correct run of Richards: they
 * specify how many times a packet is queued and how many times a task is put on
 * hold. If the actual counts differ, the implementation has a bug.
 */
const val EXPECTED_QUEUE_COUNT = 2322
const val EXPECTED_HOLD_COUNT = 928

/**
 * A simple package of data manipulated by the tasks. Besides carrying data,
 * packets form linked lists and are hence used both as data and worklists.
 */
class Packet(var link: Packet?, var id: Int, val kind: Int) {
    var a1: Int = 0
    val a2: IntArray = IntArray(DATA_SIZE)

    /** Add this packet to the end of [queue], and return the worklist. */
    fun addTo(queue: Packet?): Packet {
        link = null
        if (queue == null) return this
        var next: Packet = queue
        var peek = next.link
        while (peek != null) {
            next = peek
            peek = next.link
        }
        next.link = this
        return queue
    }
}

/** The work a [TaskControlBlock] performs when scheduled. */
abstract class Task {
    abstract fun run(packet: Packet?): TaskControlBlock?
}

/** Cycles control between the two device tasks; ends the run when [count] hits 0. */
class IdleTask(val scheduler: Scheduler, var v1: Int, var count: Int) : Task() {
    override fun run(packet: Packet?): TaskControlBlock? {
        count--
        if (count == 0) return scheduler.holdCurrent()
        return if ((v1 and 1) == 0) {
            v1 = v1 shr 1
            scheduler.release(ID_DEVICE_A)
        } else {
            v1 = (v1 shr 1) xor 0xD008
            scheduler.release(ID_DEVICE_B)
        }
    }
}

/** Suspends itself after each run to simulate waiting on an external device. */
class DeviceTask(val scheduler: Scheduler) : Task() {
    var v1: Packet? = null
    override fun run(packet: Packet?): TaskControlBlock? {
        if (packet == null) {
            val v = v1 ?: return scheduler.suspendCurrent()
            v1 = null
            return scheduler.queue(v)
        } else {
            v1 = packet
            return scheduler.holdCurrent()
        }
    }
}

/** Manipulates work packets. */
class WorkerTask(val scheduler: Scheduler, var v1: Int, var v2: Int) : Task() {
    override fun run(packet: Packet?): TaskControlBlock? {
        if (packet == null) return scheduler.suspendCurrent()
        v1 = if (v1 == ID_HANDLER_A) ID_HANDLER_B else ID_HANDLER_A
        packet.id = v1
        packet.a1 = 0
        for (i in 0 until DATA_SIZE) {
            v2++
            if (v2 > 26) v2 = 1
            packet.a2[i] = v2
        }
        return scheduler.queue(packet)
    }
}

/** Manipulates work packets and then suspends itself. */
class HandlerTask(val scheduler: Scheduler) : Task() {
    var v1: Packet? = null
    var v2: Packet? = null
    override fun run(packet: Packet?): TaskControlBlock? {
        if (packet != null) {
            if (packet.kind == KIND_WORK) v1 = packet.addTo(v1) else v2 = packet.addTo(v2)
        }
        val work = v1
        if (work != null) {
            val count = work.a1
            if (count < DATA_SIZE) {
                val dev = v2
                if (dev != null) {
                    v2 = dev.link
                    dev.a1 = work.a2[count]
                    work.a1 = count + 1
                    return scheduler.queue(dev)
                }
            } else {
                v1 = work.link
                return scheduler.queue(work)
            }
        }
        return scheduler.suspendCurrent()
    }
}

/** Manages a task and the queue of work packets associated with it. */
class TaskControlBlock(
    val link: TaskControlBlock?,
    val id: Int,
    val priority: Int,
    var queue: Packet?,
    val task: Task,
) {
    var state: Int = if (queue == null) STATE_SUSPENDED else STATE_SUSPENDED_RUNNABLE

    fun setRunning() { state = STATE_RUNNING }
    fun markAsNotHeld() { state = state and STATE_NOT_HELD }
    fun markAsHeld() { state = state or STATE_HELD }
    fun isHeldOrSuspended(): Boolean = (state and STATE_HELD) != 0 || state == STATE_SUSPENDED
    fun markAsSuspended() { state = state or STATE_SUSPENDED }
    fun markAsRunnable() { state = state or STATE_RUNNABLE }

    /** Run this task, if ready, and return the next task to run. */
    fun run(): TaskControlBlock? {
        val packet: Packet?
        if (state == STATE_SUSPENDED_RUNNABLE) {
            packet = queue
            queue = packet!!.link
            state = if (queue == null) STATE_RUNNING else STATE_RUNNABLE
        } else {
            packet = null
        }
        return task.run(packet)
    }

    /**
     * Add [packet] to this block's worklist, mark it runnable if necessary, and
     * return the next runnable task (the highest-priority one).
     */
    fun checkPriorityAdd(tcb: TaskControlBlock, packet: Packet): TaskControlBlock {
        if (queue == null) {
            queue = packet
            markAsRunnable()
            if (priority > tcb.priority) return this
        } else {
            queue = packet.addTo(queue)
        }
        return tcb
    }
}

/** Schedules a set of tasks based on their relative priorities. */
class Scheduler {
    var queueCount = 0
    var holdCount = 0
    val blocks = arrayOfNulls<TaskControlBlock>(NUMBER_OF_IDS)
    var list: TaskControlBlock? = null
    var currentTcb: TaskControlBlock? = null
    var currentId: Int = 0

    fun addIdleTask(id: Int, priority: Int, queue: Packet?, count: Int) =
        addRunningTask(id, priority, queue, IdleTask(this, 1, count))

    fun addWorkerTask(id: Int, priority: Int, queue: Packet?) =
        addTask(id, priority, queue, WorkerTask(this, ID_HANDLER_A, 0))

    fun addHandlerTask(id: Int, priority: Int, queue: Packet?) =
        addTask(id, priority, queue, HandlerTask(this))

    fun addDeviceTask(id: Int, priority: Int, queue: Packet?) =
        addTask(id, priority, queue, DeviceTask(this))

    fun addRunningTask(id: Int, priority: Int, queue: Packet?, task: Task) {
        addTask(id, priority, queue, task)
        currentTcb!!.setRunning()
    }

    fun addTask(id: Int, priority: Int, queue: Packet?, task: Task) {
        val tcb = TaskControlBlock(list, id, priority, queue, task)
        currentTcb = tcb
        list = tcb
        blocks[id] = tcb
    }

    fun schedule() {
        currentTcb = list
        while (currentTcb != null) {
            val tcb = currentTcb!!
            if (tcb.isHeldOrSuspended()) {
                currentTcb = tcb.link
            } else {
                currentId = tcb.id
                currentTcb = tcb.run()
            }
        }
    }

    fun release(id: Int): TaskControlBlock? {
        val tcb = blocks[id] ?: return null
        tcb.markAsNotHeld()
        return if (tcb.priority > currentTcb!!.priority) tcb else currentTcb
    }

    fun holdCurrent(): TaskControlBlock? {
        holdCount++
        currentTcb!!.markAsHeld()
        return currentTcb!!.link
    }

    fun suspendCurrent(): TaskControlBlock? {
        currentTcb!!.markAsSuspended()
        return currentTcb
    }

    fun queue(packet: Packet): TaskControlBlock? {
        val t = blocks[packet.id] ?: return null
        queueCount++
        packet.link = null
        packet.id = currentId
        return t.checkPriorityAdd(currentTcb!!, packet)
    }
}

/** Run one Richards simulation; returns the scheduler holding the run's counts. */
fun runRichards(): Scheduler {
    val scheduler = Scheduler()
    scheduler.addIdleTask(ID_IDLE, 0, null, COUNT)

    var queue: Packet? = Packet(null, ID_WORKER, KIND_WORK)
    queue = Packet(queue, ID_WORKER, KIND_WORK)
    scheduler.addWorkerTask(ID_WORKER, 1000, queue)

    queue = Packet(null, ID_DEVICE_A, KIND_DEVICE)
    queue = Packet(queue, ID_DEVICE_A, KIND_DEVICE)
    queue = Packet(queue, ID_DEVICE_A, KIND_DEVICE)
    scheduler.addHandlerTask(ID_HANDLER_A, 2000, queue)

    queue = Packet(null, ID_DEVICE_B, KIND_DEVICE)
    queue = Packet(queue, ID_DEVICE_B, KIND_DEVICE)
    queue = Packet(queue, ID_DEVICE_B, KIND_DEVICE)
    scheduler.addHandlerTask(ID_HANDLER_B, 3000, queue)

    scheduler.addDeviceTask(ID_DEVICE_A, 4000, null)
    scheduler.addDeviceTask(ID_DEVICE_B, 5000, null)

    scheduler.schedule()
    return scheduler
}
