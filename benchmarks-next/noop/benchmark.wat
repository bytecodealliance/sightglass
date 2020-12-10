(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i64 i32) (result i64)))
  (type (;6;) (func (param i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (import "bench" "start" (func $bench_start (type 2)))
  (import "bench" "end" (func $bench_end (type 2)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type 3)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__wasi_fd_write (type 7)))
  (func $__wasm_call_ctors (type 2)
    nop)
  (func $__original_main (type 4) (result i32)
    i32.const 1024
    call $puts
    drop
    call $bench_start
    call $bench_end
    i32.const 1088
    call $puts
    drop
    i32.const 0)
  (func $_start (type 2)
    call $__wasm_call_ctors
    call $__original_main
    call $exit
    unreachable)
  (func $__errno_location (type 4) (result i32)
    i32.const 1264)
  (func $__wasi_syscall_ret (type 0) (param i32) (result i32)
    local.get 0
    i32.eqz
    if  ;; label = @1
      i32.const 0
      return
    end
    call $__errno_location
    local.get 0
    i32.store
    i32.const -1)
  (func $_Exit (type 3) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $emscripten_memcpy_big (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 2
    if  ;; label = @1
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        local.get 2
        i32.const 508
        local.get 2
        i32.const 508
        i32.lt_u
        select
        local.tee 4
        call $memcpy
        local.set 3
        local.get 1
        i32.const 508
        i32.add
        local.set 1
        local.get 3
        i32.const 508
        i32.add
        local.set 3
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $libc_exit_fini (type 2)
    call $_fini)
  (func $exit (type 3) (param i32)
    call $_fini
    call $libc_exit_fini
    call $__stdio_exit
    local.get 0
    call $_Exit
    unreachable)
  (func $_fini (type 2)
    nop)
  (func $memcpy (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 2
    i32.const 512
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      call $emscripten_memcpy_big
      drop
      local.get 0
      return
    end
    local.get 0
    local.get 2
    i32.add
    local.set 3
    block  ;; label = @1
      local.get 0
      local.get 1
      i32.xor
      i32.const 3
      i32.and
      i32.eqz
      if  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.lt_s
          if  ;; label = @4
            local.get 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          i32.const 3
          i32.and
          i32.eqz
          if  ;; label = @4
            local.get 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          local.set 2
          loop  ;; label = @4
            local.get 2
            local.get 1
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 2
            i32.const 1
            i32.add
            local.tee 2
            local.get 3
            i32.ge_u
            br_if 1 (;@3;)
            local.get 2
            i32.const 3
            i32.and
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 3
          i32.const -4
          i32.and
          local.tee 4
          i32.const 64
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.const -64
          i32.add
          local.tee 5
          i32.gt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 2
            local.get 1
            i32.load
            i32.store
            local.get 2
            local.get 1
            i32.load offset=4
            i32.store offset=4
            local.get 2
            local.get 1
            i32.load offset=8
            i32.store offset=8
            local.get 2
            local.get 1
            i32.load offset=12
            i32.store offset=12
            local.get 2
            local.get 1
            i32.load offset=16
            i32.store offset=16
            local.get 2
            local.get 1
            i32.load offset=20
            i32.store offset=20
            local.get 2
            local.get 1
            i32.load offset=24
            i32.store offset=24
            local.get 2
            local.get 1
            i32.load offset=28
            i32.store offset=28
            local.get 2
            local.get 1
            i32.load offset=32
            i32.store offset=32
            local.get 2
            local.get 1
            i32.load offset=36
            i32.store offset=36
            local.get 2
            local.get 1
            i32.load offset=40
            i32.store offset=40
            local.get 2
            local.get 1
            i32.load offset=44
            i32.store offset=44
            local.get 2
            local.get 1
            i32.load offset=48
            i32.store offset=48
            local.get 2
            local.get 1
            i32.load offset=52
            i32.store offset=52
            local.get 2
            local.get 1
            i32.load offset=56
            i32.store offset=56
            local.get 2
            local.get 1
            i32.load offset=60
            i32.store offset=60
            local.get 1
            i32.const -64
            i32.sub
            local.set 1
            local.get 2
            i32.const -64
            i32.sub
            local.tee 2
            local.get 5
            i32.le_u
            br_if 0 (;@4;)
          end
        end
        local.get 2
        local.get 4
        i32.ge_u
        br_if 1 (;@1;)
        loop  ;; label = @3
          local.get 2
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 2
          i32.const 4
          i32.add
          local.tee 2
          local.get 4
          i32.lt_u
          br_if 0 (;@3;)
        end
        br 1 (;@1;)
      end
      local.get 3
      i32.const 4
      i32.lt_u
      if  ;; label = @2
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.const -4
      i32.add
      local.tee 4
      local.get 0
      i32.lt_u
      if  ;; label = @2
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
        local.get 2
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 2
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 2
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        local.get 4
        i32.le_u
        br_if 0 (;@2;)
      end
    end
    local.get 2
    local.get 3
    i32.lt_u
    if  ;; label = @1
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        local.get 3
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $__stdio_write (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.load offset=28
    local.tee 4
    i32.store offset=16
    local.get 0
    i32.load offset=20
    local.set 5
    local.get 3
    local.get 2
    i32.store offset=28
    local.get 3
    local.get 1
    i32.store offset=24
    local.get 3
    local.get 5
    local.get 4
    i32.sub
    local.tee 1
    i32.store offset=20
    local.get 1
    local.get 2
    i32.add
    local.set 6
    i32.const 2
    local.set 7
    local.get 3
    i32.const 16
    i32.add
    local.set 1
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=60
          local.get 3
          i32.const 16
          i32.add
          i32.const 2
          local.get 3
          i32.const 12
          i32.add
          call $__wasi_fd_write
          call $__wasi_syscall_ret
          i32.eqz
          if  ;; label = @4
            loop  ;; label = @5
              local.get 6
              local.get 3
              i32.load offset=12
              local.tee 4
              i32.eq
              br_if 2 (;@3;)
              local.get 4
              i32.const -1
              i32.le_s
              br_if 3 (;@2;)
              local.get 1
              local.get 4
              local.get 1
              i32.load offset=4
              local.tee 8
              i32.gt_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.add
              local.tee 9
              local.get 4
              local.get 8
              i32.const 0
              local.get 5
              select
              i32.sub
              local.tee 8
              local.get 9
              i32.load
              i32.add
              i32.store
              local.get 1
              i32.const 12
              i32.const 4
              local.get 5
              select
              i32.add
              local.tee 9
              local.get 9
              i32.load
              local.get 8
              i32.sub
              i32.store
              local.get 6
              local.get 4
              i32.sub
              local.set 6
              local.get 0
              i32.load offset=60
              local.get 1
              i32.const 8
              i32.add
              local.get 1
              local.get 5
              select
              local.tee 1
              local.get 7
              local.get 5
              i32.sub
              local.tee 7
              local.get 3
              i32.const 12
              i32.add
              call $__wasi_fd_write
              call $__wasi_syscall_ret
              i32.eqz
              br_if 0 (;@5;)
            end
          end
          local.get 6
          i32.const -1
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 0
        i32.load offset=44
        local.tee 1
        i32.store offset=28
        local.get 0
        local.get 1
        i32.store offset=20
        local.get 0
        local.get 1
        local.get 0
        i32.load offset=48
        i32.add
        i32.store offset=16
        local.get 2
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=28
      local.get 0
      i64.const 0
      i64.store offset=16
      local.get 0
      local.get 0
      i32.load
      i32.const 32
      i32.or
      i32.store
      i32.const 0
      local.tee 4
      local.get 7
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      drop
      local.get 2
      local.get 1
      i32.load offset=4
      i32.sub
    end
    local.set 4
    local.get 3
    i32.const 32
    i32.add
    global.set 0
    local.get 4)
  (func $__emscripten_stdout_close (type 0) (param i32) (result i32)
    i32.const 0)
  (func $__emscripten_stdout_seek (type 5) (param i32 i64 i32) (result i64)
    i64.const 0)
  (func $__lock (type 3) (param i32)
    nop)
  (func $__unlock (type 3) (param i32)
    nop)
  (func $__ofl_lock (type 4) (result i32)
    i32.const 2312
    call $__lock
    i32.const 2320)
  (func $__ofl_unlock (type 2)
    i32.const 2312
    call $__unlock)
  (func $__stdio_exit (type 2)
    (local i32)
    call $__ofl_lock
    i32.load
    local.tee 0
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0
        call $close_file
        local.get 0
        i32.load offset=56
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    i32.const 2324
    i32.load
    call $close_file
    i32.const 1256
    i32.load
    call $close_file)
  (func $close_file (type 3) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=76
      i32.const 0
      i32.ge_s
      if  ;; label = @2
        local.get 0
        call $__lockfile
        drop
      end
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=28
      i32.gt_u
      if  ;; label = @2
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=36
        call_indirect (type 1)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.ge_u
      br_if 0 (;@1;)
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
    end)
  (func $__towrite (type 0) (param i32) (result i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load8_u offset=74
    local.tee 1
    i32.const -1
    i32.add
    local.get 1
    i32.or
    i32.store8 offset=74
    local.get 0
    i32.load
    local.tee 1
    i32.const 8
    i32.and
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    local.get 0
    i64.const 0
    i64.store offset=4 align=4
    local.get 0
    local.get 0
    i32.load offset=44
    local.tee 1
    i32.store offset=28
    local.get 0
    local.get 1
    i32.store offset=20
    local.get 0
    local.get 1
    local.get 0
    i32.load offset=48
    i32.add
    i32.store offset=16
    i32.const 0)
  (func $__fwritex (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        local.get 2
        i32.load offset=16
        local.tee 3
        i32.eqz
        if  ;; label = @3
          local.get 2
          call $__towrite
          br_if 2 (;@1;)
          local.get 2
          i32.load offset=16
          local.set 3
        end
        local.get 3
        local.get 2
        i32.load offset=20
        local.tee 5
        i32.sub
        local.get 1
        i32.lt_u
      end
      if  ;; label = @2
        local.get 2
        local.get 0
        local.get 1
        local.get 2
        i32.load offset=36
        call_indirect (type 1)
        return
      end
      block  ;; label = @2
        local.get 2
        i32.load8_s offset=75
        i32.const 0
        i32.lt_s
        if  ;; label = @3
          i32.const 0
          local.set 3
          br 1 (;@2;)
        end
        local.get 1
        local.set 4
        loop  ;; label = @3
          local.get 4
          local.tee 3
          i32.eqz
          if  ;; label = @4
            i32.const 0
            local.set 3
            br 2 (;@2;)
          end
          local.get 0
          local.get 3
          i32.const -1
          i32.add
          local.tee 4
          i32.add
          i32.load8_u
          i32.const 10
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        local.get 3
        local.get 2
        i32.load offset=36
        call_indirect (type 1)
        local.tee 4
        local.get 3
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 3
        i32.add
        local.set 0
        local.get 1
        local.get 3
        i32.sub
        local.set 1
        local.get 2
        i32.load offset=20
        local.set 5
      end
      local.get 5
      local.get 0
      local.get 1
      call $memcpy
      drop
      local.get 2
      local.get 2
      i32.load offset=20
      local.get 1
      i32.add
      i32.store offset=20
      local.get 1
      local.get 3
      i32.add
      local.set 4
    end
    local.get 4)
  (func $fwrite (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 1
    local.get 2
    i32.mul
    local.set 4
    block  ;; label = @1
      local.get 3
      i32.load offset=76
      i32.const -1
      i32.le_s
      if  ;; label = @2
        local.get 0
        local.get 4
        local.get 3
        call $__fwritex
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      call $__lockfile
      local.set 5
      local.get 0
      local.get 4
      local.get 3
      call $__fwritex
      local.set 0
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      call $__unlockfile
    end
    local.get 0
    local.get 4
    i32.eq
    if  ;; label = @1
      local.get 2
      i32.const 0
      local.get 1
      select
      return
    end
    local.get 0
    local.get 1
    i32.div_u)
  (func $fputs (type 6) (param i32 i32) (result i32)
    (local i32)
    i32.const -1
    i32.const 0
    local.get 0
    i32.const 1
    local.get 0
    call $strlen
    local.tee 2
    local.get 1
    call $fwrite
    local.get 2
    i32.ne
    select)
  (func $__overflow (type 6) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store8 offset=15
    block  ;; label = @1
      local.get 0
      i32.load offset=16
      local.tee 2
      i32.eqz
      if  ;; label = @2
        i32.const -1
        local.set 2
        local.get 0
        call $__towrite
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=16
        local.set 2
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=20
        local.tee 4
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        local.tee 2
        local.get 0
        i32.load8_s offset=75
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.const 1
        i32.add
        i32.store offset=20
        local.get 4
        local.get 1
        i32.store8
        br 1 (;@1;)
      end
      i32.const -1
      local.set 2
      local.get 0
      local.get 3
      i32.const 15
      i32.add
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load8_u offset=15
      local.set 2
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 2)
  (func $puts (type 0) (param i32) (result i32)
    (local i32 i32)
    i32.const 1104
    i32.load
    local.tee 1
    i32.load offset=76
    i32.const 0
    i32.ge_s
    if  ;; label = @1
      local.get 1
      call $__lockfile
      local.set 2
    end
    block (result i32)  ;; label = @1
      i32.const -1
      local.get 0
      local.get 1
      call $fputs
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      drop
      block  ;; label = @2
        local.get 1
        i32.load8_u offset=75
        i32.const 10
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=20
        local.tee 0
        local.get 1
        i32.load offset=16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        i32.const 1
        i32.add
        i32.store offset=20
        local.get 0
        i32.const 10
        i32.store8
        i32.const 0
        br 1 (;@1;)
      end
      local.get 1
      i32.const 10
      call $__overflow
      i32.const 31
      i32.shr_s
    end
    local.set 0
    local.get 2
    if  ;; label = @1
      local.get 1
      call $__unlockfile
    end
    local.get 0)
  (func $__lockfile (type 0) (param i32) (result i32)
    i32.const 1)
  (func $__unlockfile (type 3) (param i32)
    nop)
  (func $strlen (type 0) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u
        i32.eqz
        if  ;; label = @3
          i32.const 0
          return
        end
        loop  ;; label = @3
          local.get 1
          i32.const 1
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          i32.load8_u
          br_if 0 (;@3;)
        end
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 1
        local.tee 2
        i32.const 4
        i32.add
        local.set 1
        local.get 2
        i32.load
        local.tee 3
        i32.const -1
        i32.xor
        local.get 3
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        i32.eqz
        br_if 0 (;@2;)
      end
      local.get 3
      i32.const 255
      i32.and
      i32.eqz
      if  ;; label = @2
        local.get 2
        local.get 0
        i32.sub
        return
      end
      loop  ;; label = @2
        local.get 2
        i32.load8_u offset=1
        local.set 3
        local.get 2
        i32.const 1
        i32.add
        local.tee 1
        local.set 2
        local.get 3
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub)
  (func $stackSave (type 4) (result i32)
    global.get 0)
  (func $stackRestore (type 3) (param i32)
    local.get 0
    global.set 0)
  (func $stackAlloc (type 0) (param i32) (result i32)
    (local i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 1
    global.set 0
    local.get 1)
  (func $fflush (type 0) (param i32) (result i32)
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
      i32.const 1256
      i32.load
      if  ;; label = @2
        i32.const 1256
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
  (func $__fflush_unlocked (type 0) (param i32) (result i32)
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
      call_indirect (type 1)
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
  (table (;0;) 5 5 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) (i32.const 5245216))
  (global (;1;) i32 (i32.const 2328))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "_start" (func $_start))
  (export "__errno_location" (func $__errno_location))
  (export "fflush" (func $fflush))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  (export "__data_end" (global 1))
  (elem (;0;) (i32.const 1) func $__wasm_call_ctors $__emscripten_stdout_close $__stdio_write $__emscripten_stdout_seek)
  (data (;0;) (i32.const 1024) "[noop] calls bench_start and bench_end with no intervening code\00[noop] complete\00X\04")
  (data (;1;) (i32.const 1112) "\05")
  (data (;2;) (i32.const 1124) "\02")
  (data (;3;) (i32.const 1148) "\03\00\00\00\04\00\00\00\08\05\00\00\00\04")
  (data (;4;) (i32.const 1172) "\01")
  (data (;5;) (i32.const 1187) "\0a\ff\ff\ff\ff")
  (data (;6;) (i32.const 1256) "X\04"))
