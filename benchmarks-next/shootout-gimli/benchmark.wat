(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i64 i32) (result i64)))
  (import "bench" "start" (func $bench_start (type 1)))
  (import "bench" "end" (func $bench_end (type 1)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type 0)))
  (func $__wasm_call_ctors (type 1)
    nop)
  (func $__original_main (type 2) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 40
    i32.add
    local.tee 3
    i64.const 0
    i64.store
    local.get 0
    i32.const 32
    i32.add
    local.tee 5
    i64.const 0
    i64.store
    local.get 0
    i32.const 24
    i32.add
    local.tee 4
    i64.const 0
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 0
    i64.const 0
    i64.store offset=8
    local.get 0
    i64.const 0
    i64.store
    local.get 0
    i32.const 1024
    i32.load
    call_indirect (type 0)
    call $bench_start
    local.get 0
    i32.load offset=40
    local.set 3
    local.get 0
    i32.load offset=24
    local.set 8
    local.get 0
    i32.load offset=32
    local.set 5
    local.get 0
    i32.load offset=16
    local.set 12
    local.get 0
    i32.load offset=44
    local.set 4
    local.get 0
    i32.load offset=28
    local.set 9
    local.get 0
    i32.load offset=12
    local.set 6
    local.get 0
    i32.load offset=8
    local.set 7
    local.get 0
    i32.load offset=36
    local.set 1
    local.get 0
    i32.load offset=20
    local.set 2
    local.get 0
    i32.load offset=4
    local.set 10
    local.get 0
    i32.load
    local.set 11
    loop  ;; label = @1
      i32.const 24
      local.set 14
      loop  ;; label = @2
        local.get 6
        i32.const 24
        i32.rotl
        local.tee 6
        local.get 9
        i32.const 9
        i32.rotl
        local.tee 9
        i32.xor
        local.set 16
        local.get 4
        local.get 6
        i32.or
        i32.const 1
        i32.shl
        local.set 17
        local.get 6
        local.get 4
        i32.const 1
        i32.shl
        i32.xor
        local.set 18
        local.get 4
        local.get 9
        i32.and
        i32.const 2
        i32.shl
        local.set 19
        local.get 7
        i32.const 24
        i32.rotl
        local.tee 7
        local.get 8
        i32.const 9
        i32.rotl
        local.tee 8
        i32.xor
        local.set 20
        local.get 3
        local.get 7
        i32.or
        i32.const 1
        i32.shl
        local.set 21
        local.get 7
        local.get 3
        i32.const 1
        i32.shl
        i32.xor
        local.set 22
        local.get 3
        local.get 8
        i32.and
        i32.const 2
        i32.shl
        local.set 23
        local.get 2
        i32.const 9
        i32.rotl
        local.tee 2
        local.get 1
        i32.xor
        local.get 10
        i32.const 24
        i32.rotl
        local.tee 13
        local.get 2
        i32.and
        i32.const 3
        i32.shl
        i32.xor
        local.set 10
        local.get 2
        local.get 13
        i32.xor
        local.set 24
        local.get 1
        local.get 13
        i32.or
        i32.const 1
        i32.shl
        local.set 25
        local.get 13
        local.get 1
        i32.const 1
        i32.shl
        i32.xor
        local.set 13
        local.get 1
        local.get 2
        i32.and
        i32.const 2
        i32.shl
        local.set 26
        local.get 12
        i32.const 9
        i32.rotl
        local.tee 1
        local.get 5
        i32.xor
        local.get 11
        i32.const 24
        i32.rotl
        local.tee 2
        local.get 1
        i32.and
        i32.const 3
        i32.shl
        i32.xor
        local.set 11
        local.get 1
        local.get 2
        i32.xor
        local.set 12
        local.get 2
        local.get 5
        i32.or
        i32.const 1
        i32.shl
        local.set 27
        local.get 2
        local.get 5
        i32.const 1
        i32.shl
        i32.xor
        local.set 28
        local.get 1
        local.get 5
        i32.and
        i32.const 2
        i32.shl
        local.set 5
        local.get 4
        local.get 9
        i32.xor
        local.get 6
        local.get 9
        i32.and
        i32.const 3
        i32.shl
        i32.xor
        local.tee 4
        local.set 6
        local.get 3
        local.get 8
        i32.xor
        local.get 7
        local.get 8
        i32.and
        i32.const 3
        i32.shl
        i32.xor
        local.tee 3
        local.set 7
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 14
              i32.const 3
              i32.and
              br_table 0 (;@5;) 2 (;@3;) 1 (;@4;) 2 (;@3;)
            end
            local.get 14
            i32.const -1640531712
            i32.or
            local.get 10
            i32.xor
            local.set 1
            local.get 3
            local.set 6
            local.get 4
            local.set 7
            local.get 11
            local.set 10
            local.get 1
            local.set 11
            br 1 (;@3;)
          end
          local.get 10
          local.set 6
          local.get 11
          local.set 7
          local.get 4
          local.set 10
          local.get 3
          local.set 11
        end
        local.get 16
        local.get 17
        i32.xor
        local.set 9
        local.get 18
        local.get 19
        i32.xor
        local.set 4
        local.get 20
        local.get 21
        i32.xor
        local.set 8
        local.get 22
        local.get 23
        i32.xor
        local.set 3
        local.get 24
        local.get 25
        i32.xor
        local.set 2
        local.get 13
        local.get 26
        i32.xor
        local.set 1
        local.get 12
        local.get 27
        i32.xor
        local.set 12
        local.get 5
        local.get 28
        i32.xor
        local.set 5
        local.get 14
        i32.const -1
        i32.add
        local.tee 14
        br_if 0 (;@2;)
      end
      local.get 15
      i32.const 1
      i32.add
      local.tee 15
      i32.const 10000
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 0
    local.get 5
    i32.store offset=32
    local.get 0
    local.get 12
    i32.store offset=16
    local.get 0
    local.get 11
    i32.store
    local.get 0
    local.get 1
    i32.store offset=36
    local.get 0
    local.get 2
    i32.store offset=20
    local.get 0
    local.get 10
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store offset=40
    local.get 0
    local.get 8
    i32.store offset=24
    local.get 0
    local.get 7
    i32.store offset=8
    local.get 0
    local.get 4
    i32.store offset=44
    local.get 0
    local.get 9
    i32.store offset=28
    local.get 0
    local.get 6
    i32.store offset=12
    call $bench_end
    local.get 0
    i32.const 1024
    i32.load
    call_indirect (type 0)
    local.get 0
    i32.const 48
    i32.add
    global.set 0
    i32.const 0)
  (func $_black_box (type 0) (param i32)
    nop)
  (func $_start (type 1)
    call $__wasm_call_ctors
    call $__original_main
    call $exit
    unreachable)
  (func $__errno_location (type 2) (result i32)
    i32.const 1028)
  (func $_Exit (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $libc_exit_fini (type 1)
    call $_fini)
  (func $exit (type 0) (param i32)
    call $_fini
    call $libc_exit_fini
    call $_fini
    local.get 0
    call $_Exit
    unreachable)
  (func $_fini (type 1)
    nop)
  (func $stackSave (type 2) (result i32)
    global.get 0)
  (func $stackRestore (type 0) (param i32)
    local.get 0
    global.set 0)
  (func $stackAlloc (type 3) (param i32) (result i32)
    (local i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 1
    global.set 0
    local.get 1)
  (func $__lockfile (type 3) (param i32) (result i32)
    i32.const 1)
  (func $__unlockfile (type 0) (param i32)
    nop)
  (func $__lock (type 0) (param i32)
    nop)
  (func $__unlock (type 0) (param i32)
    nop)
  (func $__ofl_lock (type 2) (result i32)
    i32.const 1032
    call $__lock
    i32.const 1040)
  (func $__ofl_unlock (type 1)
    i32.const 1032
    call $__unlock)
  (func $fflush (type 3) (param i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      if  ;; label = @2
        local.get 0
        i32.load offset=76
        i32.const -1
        i32.le_s
        if  ;; label = @3
          local.get 0
          call $__fflush_unlocked
          return
        end
        local.get 0
        call $__lockfile
        local.set 2
        local.get 0
        call $__fflush_unlocked
        local.set 1
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        call $__unlockfile
        local.get 1
        return
      end
      i32.const 1044
      i32.load
      if  ;; label = @2
        i32.const 1044
        i32.load
        call $fflush
        local.set 1
      end
      call $__ofl_lock
      i32.load
      local.tee 0
      if  ;; label = @2
        loop  ;; label = @3
          i32.const 0
          local.set 2
          local.get 0
          i32.load offset=76
          i32.const 0
          i32.ge_s
          if  ;; label = @4
            local.get 0
            call $__lockfile
            local.set 2
          end
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=28
          i32.gt_u
          if  ;; label = @4
            local.get 0
            call $__fflush_unlocked
            local.get 1
            i32.or
            local.set 1
          end
          local.get 2
          if  ;; label = @4
            local.get 0
            call $__unlockfile
          end
          local.get 0
          i32.load offset=56
          local.tee 0
          br_if 0 (;@3;)
        end
      end
      call $__ofl_unlock
    end
    local.get 1)
  (func $__fflush_unlocked (type 3) (param i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=28
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.const 0
      local.get 0
      i32.load offset=36
      call_indirect (type 4)
      drop
      local.get 0
      i32.load offset=20
      br_if 0 (;@1;)
      i32.const -1
      return
    end
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 2
    i32.lt_u
    if  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=40
      call_indirect (type 5)
      drop
    end
    local.get 0
    i32.const 0
    i32.store offset=28
    local.get 0
    i64.const 0
    i64.store offset=16
    local.get 0
    i64.const 0
    i64.store offset=4 align=4
    i32.const 0)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) (i32.const 5243936))
  (global (;1;) i32 (i32.const 1048))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "_start" (func $_start))
  (export "__errno_location" (func $__errno_location))
  (export "fflush" (func $fflush))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  (export "__data_end" (global 1))
  (elem (;0;) (i32.const 1) func $_black_box $__wasm_call_ctors)
  (data (;0;) (i32.const 1024) "\01"))
