(module $shootout-ackermann.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i64 i32) (result i64)))
  (type (;3;) (func))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (type (;12;) (func (param f64 i32) (result f64)))
  (type (;13;) (func (param i32 i32 i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32)))
  (type (;15;) (func (param i32 i64)))
  (type (;16;) (func (param i32 i32 i32 i64) (result i64)))
  (type (;17;) (func (param i32 i64 i64 i64 i64)))
  (import "bench" "start" (func $bench_start (;0;) (type 3)))
  (import "bench" "end" (func $bench_end (;1;) (type 3)))
  (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (;2;) (type 4)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func $__imported_wasi_snapshot_preview1_fd_fdstat_get (;3;) (type 5)))
  (import "wasi_snapshot_preview1" "fd_prestat_get" (func $__imported_wasi_snapshot_preview1_fd_prestat_get (;4;) (type 5)))
  (import "wasi_snapshot_preview1" "fd_prestat_dir_name" (func $__imported_wasi_snapshot_preview1_fd_prestat_dir_name (;5;) (type 1)))
  (import "wasi_snapshot_preview1" "fd_read" (func $__imported_wasi_snapshot_preview1_fd_read (;6;) (type 6)))
  (import "wasi_snapshot_preview1" "fd_seek" (func $__imported_wasi_snapshot_preview1_fd_seek (;7;) (type 7)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__imported_wasi_snapshot_preview1_fd_write (;8;) (type 6)))
  (import "wasi_snapshot_preview1" "path_open" (func $__imported_wasi_snapshot_preview1_path_open (;9;) (type 8)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;10;) (type 0)))
  (table (;0;) 6 6 funcref)
  (memory (;0;) 2)
  (global $__stack_pointer (;0;) (mut i32) i32.const 71360)
  (global $GOT.data.internal.__memory_base (;1;) i32 i32.const 0)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (elem (;0;) (i32.const 1) func $_black_box $__stdio_write $__stdio_close $__stdout_write $__stdio_seek)
  (func $__wasm_call_ctors (;11;) (type 3))
  (func $undefined_weak:__wasilibc_find_relpath_alloc (;12;) (type 9) (param i32 i32 i32 i32 i32) (result i32)
    unreachable
  )
  (func $_start (;13;) (type 3)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        global.get $GOT.data.internal.__memory_base
        i32.const 4096
        i32.add
        i32.load
        br_if 0 (;@2;)
        global.get $GOT.data.internal.__memory_base
        i32.const 4096
        i32.add
        i32.const 1
        i32.store
        call $__wasi_init_tp
        call $__wasm_call_ctors
        call $__original_main
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $ackermann (;14;) (type 5) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 1
            local.set 1
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.const -1
          i32.add
          call $ackermann
          local.set 1
        end
        local.get 0
        i32.const -1
        i32.add
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 1
    i32.add
  )
  (func $__original_main (;15;) (type 10) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        i32.const 1116
        i32.const 67108864
        i32.const 0
        call $open
        local.tee 1
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 3
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
      block ;; label = @2
        loop ;; label = @3
          local.get 1
          local.get 0
          i32.const 48
          i32.add
          local.get 3
          i32.add
          local.get 3
          i32.const 63
          i32.xor
          call $read
          local.tee 4
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.add
          local.tee 3
          i32.const 62
          i32.le_u
          br_if 0 (;@3;)
        end
      end
      local.get 1
      call $close
      drop
      local.get 0
      i32.const 48
      i32.add
      local.get 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 3
      local.get 0
      i32.const 48
      i32.add
      local.get 0
      i32.const 44
      i32.add
      i32.const 10
      call $strtol
      local.get 0
      i32.load offset=44
      local.get 0
      i32.const 48
      i32.add
      i32.eq
      select
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store offset=40
    block ;; label = @1
      block ;; label = @2
        i32.const 1087
        i32.const 67108864
        i32.const 0
        call $open
        local.tee 1
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 7
        local.set 5
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
      block ;; label = @2
        loop ;; label = @3
          local.get 1
          local.get 0
          i32.const 48
          i32.add
          local.get 3
          i32.add
          local.get 3
          i32.const 63
          i32.xor
          call $read
          local.tee 4
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.add
          local.tee 3
          i32.const 62
          i32.le_u
          br_if 0 (;@3;)
        end
      end
      local.get 1
      call $close
      drop
      local.get 0
      i32.const 48
      i32.add
      local.get 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 7
      local.get 0
      i32.const 48
      i32.add
      local.get 0
      i32.const 44
      i32.add
      i32.const 10
      call $strtol
      local.get 0
      i32.load offset=44
      local.get 0
      i32.const 48
      i32.add
      i32.eq
      select
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=36
    block ;; label = @1
      block ;; label = @2
        i32.const 1053
        i32.const 67108864
        i32.const 0
        call $open
        local.tee 1
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 11
        local.set 3
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
      block ;; label = @2
        loop ;; label = @3
          local.get 1
          local.get 0
          i32.const 48
          i32.add
          local.get 3
          i32.add
          local.get 3
          i32.const 63
          i32.xor
          call $read
          local.tee 4
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.add
          local.tee 3
          i32.const 62
          i32.le_u
          br_if 0 (;@3;)
        end
      end
      local.get 1
      call $close
      drop
      local.get 0
      i32.const 48
      i32.add
      local.get 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 11
      local.get 0
      i32.const 48
      i32.add
      local.get 0
      i32.const 44
      i32.add
      i32.const 10
      call $strtol
      local.get 0
      i32.load offset=44
      local.get 0
      i32.const 48
      i32.add
      i32.eq
      select
      local.set 3
    end
    local.get 0
    local.get 2
    i32.store offset=16
    local.get 0
    local.get 5
    i32.store offset=20
    i32.const 1195
    local.get 0
    i32.const 16
    i32.add
    call $printf
    drop
    local.get 0
    i32.const 0
    i32.store offset=48
    call $bench_start
    block ;; label = @1
      local.get 3
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      loop ;; label = @2
        local.get 0
        i32.const 40
        i32.add
        i32.const 0
        i32.load offset=3840
        call_indirect (type 0)
        local.get 0
        i32.const 36
        i32.add
        i32.const 0
        i32.load offset=3840
        call_indirect (type 0)
        local.get 0
        local.get 0
        i32.load offset=40
        local.get 0
        i32.load offset=36
        call $ackermann
        i32.store offset=48
        local.get 0
        i32.const 48
        i32.add
        i32.const 0
        i32.load offset=3840
        call_indirect (type 0)
        local.get 3
        i32.const -1
        i32.add
        local.tee 3
        br_if 0 (;@2;)
      end
    end
    call $bench_end
    local.get 0
    local.get 0
    i32.load offset=48
    i32.store
    i32.const 1170
    local.get 0
    call $printf
    drop
    local.get 0
    i32.const 112
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $_black_box (;16;) (type 0) (param i32))
  (func $read (;17;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 3
        i32.const 8
        i32.add
        i32.const 1
        local.get 3
        i32.const 4
        i32.add
        call $__wasi_fd_read
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.const 8
        local.get 2
        local.get 2
        i32.const 76
        i32.eq
        select
        i32.store offset=4100
        i32.const -1
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=4
      local.set 2
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $close (;18;) (type 4) (param i32) (result i32)
    call $__wasilibc_populate_preopens
    block ;; label = @1
      local.get 0
      call $__wasi_fd_close
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.get 0
    i32.store offset=4100
    i32.const -1
  )
  (func $__wasi_fd_close (;19;) (type 4) (param i32) (result i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_fd_close
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_fdstat_get (;20;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_fd_fdstat_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_prestat_get (;21;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_fd_prestat_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_prestat_dir_name (;22;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $__imported_wasi_snapshot_preview1_fd_prestat_dir_name
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_read (;23;) (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_read
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_seek (;24;) (type 7) (param i32 i64 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_seek
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_write (;25;) (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_write
    i32.const 65535
    i32.and
  )
  (func $__wasi_path_open (;26;) (type 11) (param i32 i32 i32 i32 i64 i64 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 2
    call $strlen
    local.get 3
    local.get 4
    local.get 5
    local.get 6
    local.get 7
    call $__imported_wasi_snapshot_preview1_path_open
    i32.const 65535
    i32.and
  )
  (func $__wasi_proc_exit (;27;) (type 0) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable
  )
  (func $__wasilibc_nocwd_openat_nomode (;28;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 503316480
            i32.and
            i32.const -33554432
            i32.add
            i32.const 25
            i32.shr_u
            local.tee 4
            i32.const 9
            i32.gt_u
            br_if 0 (;@4;)
            i32.const 1
            local.get 4
            i32.shl
            local.tee 4
            i32.const 642
            i32.and
            br_if 1 (;@3;)
            i64.const -4211012
            local.set 5
            local.get 4
            i32.const 9
            i32.and
            br_if 2 (;@2;)
          end
          i32.const 0
          i32.const 28
          i32.store offset=4100
          i32.const -1
          local.set 4
          br 2 (;@1;)
        end
        i64.const -4194626
        i64.const -4211012
        local.get 2
        i32.const 67108864
        i32.and
        select
        local.tee 5
        i64.const 4194625
        i64.or
        local.get 5
        local.get 2
        i32.const 268435456
        i32.and
        select
        local.set 5
      end
      block ;; label = @2
        local.get 0
        local.get 3
        i32.const 8
        i32.add
        call $__wasi_fd_fdstat_get
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 4
        i32.store offset=4100
        i32.const -1
        local.set 4
        br 1 (;@1;)
      end
      i32.const -1
      local.set 4
      block ;; label = @2
        local.get 0
        local.get 2
        i32.const -1
        i32.xor
        i32.const 24
        i32.shr_u
        i32.const 1
        i32.and
        local.get 1
        local.get 2
        i32.const 12
        i32.shr_u
        i32.const 4095
        i32.and
        local.get 3
        i64.load offset=24
        local.tee 6
        local.get 5
        i64.and
        local.get 6
        local.get 2
        i32.const 4095
        i32.and
        local.get 3
        i32.const 4
        i32.add
        call $__wasi_path_open
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i32.store offset=4100
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=4
      local.set 4
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $malloc (;29;) (type 4) (param i32) (result i32)
    local.get 0
    call $dlmalloc
  )
  (func $dlmalloc (;30;) (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=4128
                              local.tee 2
                              br_if 0 (;@13;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=4576
                                local.tee 3
                                br_if 0 (;@14;)
                                i32.const 0
                                i64.const -1
                                i64.store offset=4588 align=4
                                i32.const 0
                                i64.const 281474976776192
                                i64.store offset=4580 align=4
                                i32.const 0
                                local.get 1
                                i32.const 8
                                i32.add
                                i32.const -16
                                i32.and
                                i32.const 1431655768
                                i32.xor
                                local.tee 3
                                i32.store offset=4576
                                i32.const 0
                                i32.const 0
                                i32.store offset=4596
                                i32.const 0
                                i32.const 0
                                i32.store offset=4548
                              end
                              i32.const 131072
                              i32.const 71360
                              i32.lt_u
                              br_if 1 (;@12;)
                              i32.const 0
                              local.set 2
                              i32.const 131072
                              i32.const 71360
                              i32.sub
                              i32.const 89
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 71360
                              i32.store offset=4552
                              i32.const 0
                              i32.const 71360
                              i32.store offset=4120
                              i32.const 0
                              local.get 3
                              i32.store offset=4140
                              i32.const 0
                              i32.const -1
                              i32.store offset=4136
                              i32.const 0
                              i32.const 131072
                              i32.const 71360
                              i32.sub
                              local.tee 3
                              i32.store offset=4556
                              i32.const 0
                              local.get 3
                              i32.store offset=4540
                              i32.const 0
                              local.get 3
                              i32.store offset=4536
                              loop ;; label = @14
                                local.get 4
                                i32.const 4164
                                i32.add
                                local.get 4
                                i32.const 4152
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 4
                                i32.const 4144
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 4
                                i32.const 4156
                                i32.add
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 4172
                                i32.add
                                local.get 4
                                i32.const 4160
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 5
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 4180
                                i32.add
                                local.get 4
                                i32.const 4168
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 4176
                                i32.add
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 32
                                i32.add
                                local.tee 4
                                i32.const 256
                                i32.ne
                                br_if 0 (;@14;)
                              end
                              i32.const 131072
                              i32.const -52
                              i32.add
                              i32.const 56
                              i32.store
                              i32.const 0
                              i32.const 0
                              i32.load offset=4592
                              i32.store offset=4132
                              i32.const 0
                              i32.const 71360
                              i32.const -8
                              i32.const 71360
                              i32.sub
                              i32.const 15
                              i32.and
                              local.tee 4
                              i32.add
                              local.tee 2
                              i32.store offset=4128
                              i32.const 0
                              i32.const 131072
                              i32.const 71360
                              i32.sub
                              local.get 4
                              i32.sub
                              i32.const -56
                              i32.add
                              local.tee 4
                              i32.store offset=4116
                              local.get 2
                              local.get 4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 0
                                i32.const 236
                                i32.gt_u
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=4104
                                  local.tee 6
                                  i32.const 16
                                  local.get 0
                                  i32.const 19
                                  i32.add
                                  i32.const 496
                                  i32.and
                                  local.get 0
                                  i32.const 11
                                  i32.lt_u
                                  select
                                  local.tee 5
                                  i32.const 3
                                  i32.shr_u
                                  local.tee 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 3
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 4
                                      i32.const 1
                                      i32.and
                                      local.get 3
                                      i32.or
                                      i32.const 1
                                      i32.xor
                                      local.tee 5
                                      i32.const 3
                                      i32.shl
                                      local.tee 3
                                      i32.const 4144
                                      i32.add
                                      local.tee 4
                                      local.get 3
                                      i32.load offset=4152
                                      local.tee 3
                                      i32.load offset=8
                                      local.tee 0
                                      i32.ne
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 5
                                      i32.rotl
                                      i32.and
                                      i32.store offset=4104
                                      br 1 (;@16;)
                                    end
                                    local.get 4
                                    local.get 0
                                    i32.store offset=8
                                    local.get 0
                                    local.get 4
                                    i32.store offset=12
                                  end
                                  local.get 3
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  local.get 3
                                  local.get 5
                                  i32.const 3
                                  i32.shl
                                  local.tee 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  local.tee 3
                                  local.get 3
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  br 14 (;@1;)
                                end
                                local.get 5
                                i32.const 0
                                i32.load offset=4112
                                local.tee 7
                                i32.le_u
                                br_if 1 (;@13;)
                                block ;; label = @15
                                  local.get 4
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 4
                                      local.get 3
                                      i32.shl
                                      i32.const 2
                                      local.get 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 0
                                      local.get 4
                                      i32.sub
                                      i32.or
                                      i32.and
                                      i32.ctz
                                      local.tee 3
                                      i32.const 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 4144
                                      i32.add
                                      local.tee 0
                                      local.get 4
                                      i32.load offset=4152
                                      local.tee 4
                                      i32.load offset=8
                                      local.tee 8
                                      i32.ne
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 3
                                      i32.rotl
                                      i32.and
                                      local.tee 6
                                      i32.store offset=4104
                                      br 1 (;@16;)
                                    end
                                    local.get 0
                                    local.get 8
                                    i32.store offset=8
                                    local.get 8
                                    local.get 0
                                    i32.store offset=12
                                  end
                                  local.get 4
                                  local.get 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 4
                                  local.get 3
                                  i32.const 3
                                  i32.shl
                                  local.tee 3
                                  i32.add
                                  local.get 3
                                  local.get 5
                                  i32.sub
                                  local.tee 0
                                  i32.store
                                  local.get 4
                                  local.get 5
                                  i32.add
                                  local.tee 8
                                  local.get 0
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  block ;; label = @16
                                    local.get 7
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 7
                                    i32.const -8
                                    i32.and
                                    i32.const 4144
                                    i32.add
                                    local.set 5
                                    i32.const 0
                                    i32.load offset=4124
                                    local.set 3
                                    block ;; label = @17
                                      block ;; label = @18
                                        local.get 6
                                        i32.const 1
                                        local.get 7
                                        i32.const 3
                                        i32.shr_u
                                        i32.shl
                                        local.tee 9
                                        i32.and
                                        br_if 0 (;@18;)
                                        i32.const 0
                                        local.get 6
                                        local.get 9
                                        i32.or
                                        i32.store offset=4104
                                        local.get 5
                                        local.set 9
                                        br 1 (;@17;)
                                      end
                                      local.get 5
                                      i32.load offset=8
                                      local.set 9
                                    end
                                    local.get 9
                                    local.get 3
                                    i32.store offset=12
                                    local.get 5
                                    local.get 3
                                    i32.store offset=8
                                    local.get 3
                                    local.get 5
                                    i32.store offset=12
                                    local.get 3
                                    local.get 9
                                    i32.store offset=8
                                  end
                                  local.get 4
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  i32.const 0
                                  local.get 8
                                  i32.store offset=4124
                                  i32.const 0
                                  local.get 0
                                  i32.store offset=4112
                                  br 14 (;@1;)
                                end
                                i32.const 0
                                i32.load offset=4108
                                local.tee 10
                                i32.eqz
                                br_if 1 (;@13;)
                                local.get 10
                                i32.ctz
                                i32.const 2
                                i32.shl
                                i32.load offset=4408
                                local.tee 8
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.set 3
                                local.get 8
                                local.set 0
                                block ;; label = @15
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 0
                                      i32.load offset=16
                                      local.tee 4
                                      br_if 0 (;@17;)
                                      local.get 0
                                      i32.load offset=20
                                      local.tee 4
                                      i32.eqz
                                      br_if 2 (;@15;)
                                    end
                                    local.get 4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 0
                                    local.get 3
                                    local.get 0
                                    local.get 3
                                    i32.lt_u
                                    local.tee 0
                                    select
                                    local.set 3
                                    local.get 4
                                    local.get 8
                                    local.get 0
                                    select
                                    local.set 8
                                    local.get 4
                                    local.set 0
                                    br 0 (;@16;)
                                  end
                                end
                                local.get 8
                                i32.load offset=24
                                local.set 2
                                block ;; label = @15
                                  local.get 8
                                  i32.load offset=12
                                  local.tee 4
                                  local.get 8
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 8
                                  i32.load offset=8
                                  local.tee 0
                                  local.get 4
                                  i32.store offset=12
                                  local.get 4
                                  local.get 0
                                  i32.store offset=8
                                  br 13 (;@2;)
                                end
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 8
                                    i32.load offset=20
                                    local.tee 0
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 8
                                    i32.const 20
                                    i32.add
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  local.get 8
                                  i32.load offset=16
                                  local.tee 0
                                  i32.eqz
                                  br_if 4 (;@11;)
                                  local.get 8
                                  i32.const 16
                                  i32.add
                                  local.set 9
                                end
                                loop ;; label = @15
                                  local.get 9
                                  local.set 11
                                  local.get 0
                                  local.tee 4
                                  i32.const 20
                                  i32.add
                                  local.set 9
                                  local.get 4
                                  i32.load offset=20
                                  local.tee 0
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 16
                                  i32.add
                                  local.set 9
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 0
                                  br_if 0 (;@15;)
                                end
                                local.get 11
                                i32.const 0
                                i32.store
                                br 12 (;@2;)
                              end
                              i32.const -1
                              local.set 5
                              local.get 0
                              i32.const -65
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 19
                              i32.add
                              local.tee 4
                              i32.const -16
                              i32.and
                              local.set 5
                              i32.const 0
                              i32.load offset=4108
                              local.tee 10
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 31
                              local.set 7
                              block ;; label = @14
                                local.get 0
                                i32.const 16777196
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 38
                                local.get 4
                                i32.const 8
                                i32.shr_u
                                i32.clz
                                local.tee 4
                                i32.sub
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.get 4
                                i32.const 1
                                i32.shl
                                i32.sub
                                i32.const 62
                                i32.add
                                local.set 7
                              end
                              i32.const 0
                              local.get 5
                              i32.sub
                              local.set 3
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 7
                                      i32.const 2
                                      i32.shl
                                      i32.load offset=4408
                                      local.tee 0
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 4
                                      i32.const 0
                                      local.set 9
                                      br 1 (;@16;)
                                    end
                                    i32.const 0
                                    local.set 4
                                    local.get 5
                                    i32.const 0
                                    i32.const 25
                                    local.get 7
                                    i32.const 1
                                    i32.shr_u
                                    i32.sub
                                    local.get 7
                                    i32.const 31
                                    i32.eq
                                    select
                                    i32.shl
                                    local.set 8
                                    i32.const 0
                                    local.set 9
                                    loop ;; label = @17
                                      block ;; label = @18
                                        local.get 0
                                        i32.load offset=4
                                        i32.const -8
                                        i32.and
                                        local.get 5
                                        i32.sub
                                        local.tee 6
                                        local.get 3
                                        i32.ge_u
                                        br_if 0 (;@18;)
                                        local.get 6
                                        local.set 3
                                        local.get 0
                                        local.set 9
                                        local.get 6
                                        br_if 0 (;@18;)
                                        i32.const 0
                                        local.set 3
                                        local.get 0
                                        local.set 9
                                        local.get 0
                                        local.set 4
                                        br 3 (;@15;)
                                      end
                                      local.get 4
                                      local.get 0
                                      i32.load offset=20
                                      local.tee 6
                                      local.get 6
                                      local.get 0
                                      local.get 8
                                      i32.const 29
                                      i32.shr_u
                                      i32.const 4
                                      i32.and
                                      i32.add
                                      i32.load offset=16
                                      local.tee 11
                                      i32.eq
                                      select
                                      local.get 4
                                      local.get 6
                                      select
                                      local.set 4
                                      local.get 8
                                      i32.const 1
                                      i32.shl
                                      local.set 8
                                      local.get 11
                                      local.set 0
                                      local.get 11
                                      br_if 0 (;@17;)
                                    end
                                  end
                                  block ;; label = @16
                                    local.get 4
                                    local.get 9
                                    i32.or
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 9
                                    i32.const 2
                                    local.get 7
                                    i32.shl
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.or
                                    local.get 10
                                    i32.and
                                    local.tee 4
                                    i32.eqz
                                    br_if 3 (;@13;)
                                    local.get 4
                                    i32.ctz
                                    i32.const 2
                                    i32.shl
                                    i32.load offset=4408
                                    local.set 4
                                  end
                                  local.get 4
                                  i32.eqz
                                  br_if 1 (;@14;)
                                end
                                loop ;; label = @15
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 5
                                  i32.sub
                                  local.tee 6
                                  local.get 3
                                  i32.lt_u
                                  local.set 8
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=16
                                    local.tee 0
                                    br_if 0 (;@16;)
                                    local.get 4
                                    i32.load offset=20
                                    local.set 0
                                  end
                                  local.get 6
                                  local.get 3
                                  local.get 8
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 9
                                  local.get 8
                                  select
                                  local.set 9
                                  local.get 0
                                  local.set 4
                                  local.get 0
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 9
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 3
                              i32.const 0
                              i32.load offset=4112
                              local.get 5
                              i32.sub
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=24
                              local.set 11
                              block ;; label = @14
                                local.get 9
                                i32.load offset=12
                                local.tee 4
                                local.get 9
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 9
                                i32.load offset=8
                                local.tee 0
                                local.get 4
                                i32.store offset=12
                                local.get 4
                                local.get 0
                                i32.store offset=8
                                br 11 (;@3;)
                              end
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 9
                                  i32.load offset=20
                                  local.tee 0
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 9
                                  i32.const 20
                                  i32.add
                                  local.set 8
                                  br 1 (;@14;)
                                end
                                local.get 9
                                i32.load offset=16
                                local.tee 0
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 8
                              end
                              loop ;; label = @14
                                local.get 8
                                local.set 6
                                local.get 0
                                local.tee 4
                                i32.const 20
                                i32.add
                                local.set 8
                                local.get 4
                                i32.load offset=20
                                local.tee 0
                                br_if 0 (;@14;)
                                local.get 4
                                i32.const 16
                                i32.add
                                local.set 8
                                local.get 4
                                i32.load offset=16
                                local.tee 0
                                br_if 0 (;@14;)
                              end
                              local.get 6
                              i32.const 0
                              i32.store
                              br 10 (;@3;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=4112
                              local.tee 4
                              local.get 5
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=4124
                              local.set 3
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 4
                                  local.get 5
                                  i32.sub
                                  local.tee 0
                                  i32.const 16
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  local.tee 8
                                  local.get 0
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 4
                                  i32.add
                                  local.get 0
                                  i32.store
                                  local.get 3
                                  local.get 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  br 1 (;@14;)
                                end
                                local.get 3
                                local.get 4
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.tee 4
                                local.get 4
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                i32.const 0
                                local.set 8
                                i32.const 0
                                local.set 0
                              end
                              i32.const 0
                              local.get 0
                              i32.store offset=4112
                              i32.const 0
                              local.get 8
                              i32.store offset=4124
                              local.get 3
                              i32.const 8
                              i32.add
                              local.set 4
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=4116
                              local.tee 0
                              local.get 5
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 2
                              local.get 5
                              i32.add
                              local.tee 4
                              local.get 0
                              local.get 5
                              i32.sub
                              local.tee 3
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.get 4
                              i32.store offset=4128
                              i32.const 0
                              local.get 3
                              i32.store offset=4116
                              local.get 2
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 2
                              i32.const 8
                              i32.add
                              local.set 4
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=4576
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                i32.load offset=4584
                                local.set 3
                                br 1 (;@13;)
                              end
                              i32.const 0
                              i64.const -1
                              i64.store offset=4588 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=4580 align=4
                              i32.const 0
                              local.get 1
                              i32.const 12
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              i32.store offset=4576
                              i32.const 0
                              i32.const 0
                              i32.store offset=4596
                              i32.const 0
                              i32.const 0
                              i32.store offset=4548
                              i32.const 65536
                              local.set 3
                            end
                            i32.const 0
                            local.set 4
                            block ;; label = @13
                              local.get 3
                              local.get 5
                              i32.const 71
                              i32.add
                              local.tee 11
                              i32.add
                              local.tee 8
                              i32.const 0
                              local.get 3
                              i32.sub
                              local.tee 6
                              i32.and
                              local.tee 9
                              local.get 5
                              i32.gt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 48
                              i32.store offset=4100
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=4544
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@13;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=4536
                                local.tee 3
                                local.get 9
                                i32.add
                                local.tee 7
                                local.get 3
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 7
                                local.get 4
                                i32.le_u
                                br_if 1 (;@13;)
                              end
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 48
                              i32.store offset=4100
                              br 12 (;@1;)
                            end
                            i32.const 0
                            i32.load8_u offset=4548
                            i32.const 4
                            i32.and
                            br_if 5 (;@7;)
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 2
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  i32.const 4552
                                  local.set 4
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 2
                                      local.get 4
                                      i32.load
                                      local.tee 3
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      local.get 2
                                      local.get 3
                                      local.get 4
                                      i32.load offset=4
                                      i32.add
                                      i32.lt_u
                                      br_if 3 (;@14;)
                                    end
                                    local.get 4
                                    i32.load offset=8
                                    local.tee 4
                                    br_if 0 (;@16;)
                                  end
                                end
                                i32.const 0
                                call $sbrk
                                local.tee 8
                                i32.const -1
                                i32.eq
                                br_if 6 (;@8;)
                                local.get 9
                                local.set 6
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=4580
                                  local.tee 4
                                  i32.const -1
                                  i32.add
                                  local.tee 3
                                  local.get 8
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 9
                                  local.get 8
                                  i32.sub
                                  local.get 3
                                  local.get 8
                                  i32.add
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.add
                                  local.set 6
                                end
                                local.get 6
                                local.get 5
                                i32.le_u
                                br_if 6 (;@8;)
                                local.get 6
                                i32.const 2147483646
                                i32.gt_u
                                br_if 6 (;@8;)
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=4544
                                  local.tee 4
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  i32.load offset=4536
                                  local.tee 3
                                  local.get 6
                                  i32.add
                                  local.tee 0
                                  local.get 3
                                  i32.le_u
                                  br_if 7 (;@8;)
                                  local.get 0
                                  local.get 4
                                  i32.gt_u
                                  br_if 7 (;@8;)
                                end
                                local.get 6
                                call $sbrk
                                local.tee 4
                                local.get 8
                                i32.ne
                                br_if 1 (;@13;)
                                br 8 (;@6;)
                              end
                              local.get 8
                              local.get 0
                              i32.sub
                              local.get 6
                              i32.and
                              local.tee 6
                              i32.const 2147483646
                              i32.gt_u
                              br_if 5 (;@8;)
                              local.get 6
                              call $sbrk
                              local.tee 8
                              local.get 4
                              i32.load
                              local.get 4
                              i32.load offset=4
                              i32.add
                              i32.eq
                              br_if 4 (;@9;)
                              local.get 8
                              local.set 4
                            end
                            block ;; label = @13
                              local.get 6
                              local.get 5
                              i32.const 72
                              i32.add
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const -1
                              i32.eq
                              br_if 0 (;@13;)
                              block ;; label = @14
                                local.get 11
                                local.get 6
                                i32.sub
                                i32.const 0
                                i32.load offset=4584
                                local.tee 3
                                i32.add
                                i32.const 0
                                local.get 3
                                i32.sub
                                i32.and
                                local.tee 3
                                i32.const 2147483646
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 4
                                local.set 8
                                br 8 (;@6;)
                              end
                              block ;; label = @14
                                local.get 3
                                call $sbrk
                                i32.const -1
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 3
                                local.get 6
                                i32.add
                                local.set 6
                                local.get 4
                                local.set 8
                                br 8 (;@6;)
                              end
                              i32.const 0
                              local.get 6
                              i32.sub
                              call $sbrk
                              drop
                              br 5 (;@8;)
                            end
                            local.get 4
                            local.set 8
                            local.get 4
                            i32.const -1
                            i32.ne
                            br_if 6 (;@6;)
                            br 4 (;@8;)
                          end
                          unreachable
                        end
                        i32.const 0
                        local.set 4
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.set 4
                      br 6 (;@3;)
                    end
                    local.get 8
                    i32.const -1
                    i32.ne
                    br_if 2 (;@6;)
                  end
                  i32.const 0
                  i32.const 0
                  i32.load offset=4548
                  i32.const 4
                  i32.or
                  i32.store offset=4548
                end
                local.get 9
                i32.const 2147483646
                i32.gt_u
                br_if 1 (;@5;)
                local.get 9
                call $sbrk
                local.set 8
                i32.const 0
                call $sbrk
                local.set 4
                local.get 8
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                local.get 4
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                local.get 8
                local.get 4
                i32.ge_u
                br_if 1 (;@5;)
                local.get 4
                local.get 8
                i32.sub
                local.tee 6
                local.get 5
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@5;)
              end
              i32.const 0
              i32.const 0
              i32.load offset=4536
              local.get 6
              i32.add
              local.tee 4
              i32.store offset=4536
              block ;; label = @6
                local.get 4
                i32.const 0
                i32.load offset=4540
                i32.le_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 4
                i32.store offset=4540
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=4128
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 4552
                      local.set 4
                      loop ;; label = @10
                        local.get 8
                        local.get 4
                        i32.load
                        local.tee 0
                        local.get 4
                        i32.load offset=4
                        local.tee 9
                        i32.add
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@10;)
                        br 3 (;@7;)
                      end
                    end
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=4120
                        local.tee 4
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 4
                        i32.ge_u
                        br_if 1 (;@9;)
                      end
                      i32.const 0
                      local.get 8
                      i32.store offset=4120
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 6
                    i32.store offset=4556
                    i32.const 0
                    local.get 8
                    i32.store offset=4552
                    i32.const 0
                    i32.const -1
                    i32.store offset=4136
                    i32.const 0
                    i32.const 0
                    i32.load offset=4576
                    i32.store offset=4140
                    i32.const 0
                    i32.const 0
                    i32.store offset=4564
                    loop ;; label = @9
                      local.get 4
                      i32.const 4164
                      i32.add
                      local.get 4
                      i32.const 4152
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 4144
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 4
                      i32.const 4156
                      i32.add
                      local.get 0
                      i32.store
                      local.get 4
                      i32.const 4172
                      i32.add
                      local.get 4
                      i32.const 4160
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 0
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 4180
                      i32.add
                      local.get 4
                      i32.const 4168
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 0
                      i32.store
                      local.get 4
                      i32.const 4176
                      i32.add
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 32
                      i32.add
                      local.tee 4
                      i32.const 256
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 8
                    i32.const -8
                    local.get 8
                    i32.sub
                    i32.const 15
                    i32.and
                    local.tee 4
                    i32.add
                    local.tee 3
                    local.get 6
                    i32.const -56
                    i32.add
                    local.tee 0
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=4592
                    i32.store offset=4132
                    i32.const 0
                    local.get 4
                    i32.store offset=4116
                    i32.const 0
                    local.get 3
                    i32.store offset=4128
                    local.get 8
                    local.get 0
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 2 (;@6;)
                  end
                  local.get 3
                  local.get 8
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 0
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=12
                  i32.const 8
                  i32.and
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const -8
                  local.get 3
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee 0
                  i32.add
                  local.tee 8
                  i32.const 0
                  i32.load offset=4116
                  local.get 6
                  i32.add
                  local.tee 11
                  local.get 0
                  i32.sub
                  local.tee 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 4
                  local.get 9
                  local.get 6
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=4592
                  i32.store offset=4132
                  i32.const 0
                  local.get 0
                  i32.store offset=4116
                  i32.const 0
                  local.get 8
                  i32.store offset=4128
                  local.get 3
                  local.get 11
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 1 (;@6;)
                end
                block ;; label = @7
                  local.get 8
                  i32.const 0
                  i32.load offset=4120
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 8
                  i32.store offset=4120
                end
                local.get 8
                local.get 6
                i32.add
                local.set 0
                i32.const 4552
                local.set 4
                block ;; label = @7
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 4
                      i32.load
                      local.tee 9
                      local.get 0
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 4
                      i32.load offset=8
                      local.tee 4
                      br_if 0 (;@9;)
                      br 2 (;@7;)
                    end
                  end
                  local.get 4
                  i32.load8_u offset=12
                  i32.const 8
                  i32.and
                  i32.eqz
                  br_if 3 (;@4;)
                end
                i32.const 4552
                local.set 4
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 3
                      local.get 4
                      i32.load
                      local.tee 0
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 3
                      local.get 0
                      local.get 4
                      i32.load offset=4
                      i32.add
                      local.tee 0
                      i32.lt_u
                      br_if 2 (;@7;)
                    end
                    local.get 4
                    i32.load offset=8
                    local.set 4
                    br 0 (;@8;)
                  end
                end
                local.get 8
                i32.const -8
                local.get 8
                i32.sub
                i32.const 15
                i32.and
                local.tee 4
                i32.add
                local.tee 11
                local.get 6
                i32.const -56
                i32.add
                local.tee 9
                local.get 4
                i32.sub
                local.tee 4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 8
                local.get 9
                i32.add
                i32.const 56
                i32.store offset=4
                local.get 3
                local.get 0
                i32.const 55
                local.get 0
                i32.sub
                i32.const 15
                i32.and
                i32.add
                i32.const -63
                i32.add
                local.tee 9
                local.get 9
                local.get 3
                i32.const 16
                i32.add
                i32.lt_u
                select
                local.tee 9
                i32.const 35
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=4592
                i32.store offset=4132
                i32.const 0
                local.get 4
                i32.store offset=4116
                i32.const 0
                local.get 11
                i32.store offset=4128
                local.get 9
                i32.const 16
                i32.add
                i32.const 0
                i64.load offset=4560 align=4
                i64.store align=4
                local.get 9
                i32.const 0
                i64.load offset=4552 align=4
                i64.store offset=8 align=4
                i32.const 0
                local.get 9
                i32.const 8
                i32.add
                i32.store offset=4560
                i32.const 0
                local.get 6
                i32.store offset=4556
                i32.const 0
                local.get 8
                i32.store offset=4552
                i32.const 0
                i32.const 0
                i32.store offset=4564
                local.get 9
                i32.const 36
                i32.add
                local.set 4
                loop ;; label = @7
                  local.get 4
                  i32.const 7
                  i32.store
                  local.get 4
                  i32.const 4
                  i32.add
                  local.tee 4
                  local.get 0
                  i32.lt_u
                  br_if 0 (;@7;)
                end
                local.get 9
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 9
                local.get 9
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 9
                local.get 9
                local.get 3
                i32.sub
                local.tee 8
                i32.store
                local.get 3
                local.get 8
                i32.const 1
                i32.or
                i32.store offset=4
                block ;; label = @7
                  block ;; label = @8
                    local.get 8
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const -8
                    i32.and
                    i32.const 4144
                    i32.add
                    local.set 4
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=4104
                        local.tee 0
                        i32.const 1
                        local.get 8
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 8
                        i32.and
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 0
                        local.get 8
                        i32.or
                        i32.store offset=4104
                        local.get 4
                        local.set 0
                        br 1 (;@9;)
                      end
                      local.get 4
                      i32.load offset=8
                      local.set 0
                    end
                    local.get 0
                    local.get 3
                    i32.store offset=12
                    local.get 4
                    local.get 3
                    i32.store offset=8
                    i32.const 12
                    local.set 8
                    i32.const 8
                    local.set 9
                    br 1 (;@7;)
                  end
                  i32.const 31
                  local.set 4
                  block ;; label = @8
                    local.get 8
                    i32.const 16777215
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 38
                    local.get 8
                    i32.const 8
                    i32.shr_u
                    i32.clz
                    local.tee 4
                    i32.sub
                    i32.shr_u
                    i32.const 1
                    i32.and
                    local.get 4
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 62
                    i32.add
                    local.set 4
                  end
                  local.get 3
                  local.get 4
                  i32.store offset=28
                  local.get 3
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 4
                  i32.const 2
                  i32.shl
                  i32.const 4408
                  i32.add
                  local.set 0
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=4108
                        local.tee 9
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 6
                        i32.and
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 3
                        i32.store
                        i32.const 0
                        local.get 9
                        local.get 6
                        i32.or
                        i32.store offset=4108
                        local.get 3
                        local.get 0
                        i32.store offset=24
                        br 1 (;@9;)
                      end
                      local.get 8
                      i32.const 0
                      i32.const 25
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 4
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 4
                      local.get 0
                      i32.load
                      local.set 9
                      loop ;; label = @10
                        local.get 9
                        local.tee 0
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 8
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 4
                        i32.const 29
                        i32.shr_u
                        local.set 9
                        local.get 4
                        i32.const 1
                        i32.shl
                        local.set 4
                        local.get 0
                        local.get 9
                        i32.const 4
                        i32.and
                        i32.add
                        local.tee 6
                        i32.load offset=16
                        local.tee 9
                        br_if 0 (;@10;)
                      end
                      local.get 6
                      i32.const 16
                      i32.add
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 0
                      i32.store offset=24
                    end
                    i32.const 8
                    local.set 8
                    i32.const 12
                    local.set 9
                    local.get 3
                    local.set 0
                    local.get 3
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=8
                  local.set 4
                  local.get 0
                  local.get 3
                  i32.store offset=8
                  local.get 4
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  i32.const 0
                  local.set 4
                  i32.const 24
                  local.set 8
                  i32.const 12
                  local.set 9
                end
                local.get 3
                local.get 9
                i32.add
                local.get 0
                i32.store
                local.get 3
                local.get 8
                i32.add
                local.get 4
                i32.store
              end
              i32.const 0
              i32.load offset=4116
              local.tee 4
              local.get 5
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=4128
              local.tee 3
              local.get 5
              i32.add
              local.tee 0
              local.get 4
              local.get 5
              i32.sub
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.get 4
              i32.store offset=4116
              i32.const 0
              local.get 0
              i32.store offset=4128
              local.get 3
              local.get 5
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 3
              i32.const 8
              i32.add
              local.set 4
              br 4 (;@1;)
            end
            i32.const 0
            local.set 4
            i32.const 0
            i32.const 48
            i32.store offset=4100
            br 3 (;@1;)
          end
          local.get 4
          local.get 8
          i32.store
          local.get 4
          local.get 4
          i32.load offset=4
          local.get 6
          i32.add
          i32.store offset=4
          local.get 8
          local.get 9
          local.get 5
          call $prepend_alloc
          local.set 4
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 11
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 8
              i32.const 2
              i32.shl
              local.tee 0
              i32.load offset=4408
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              i32.const 4408
              i32.add
              local.get 4
              i32.store
              local.get 4
              br_if 1 (;@4;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 8
              i32.rotl
              i32.and
              local.tee 10
              i32.store offset=4108
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 11
                i32.load offset=16
                local.get 9
                i32.ne
                br_if 0 (;@6;)
                local.get 11
                local.get 4
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 11
              local.get 4
              i32.store offset=20
            end
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 4
          local.get 11
          i32.store offset=24
          block ;; label = @4
            local.get 9
            i32.load offset=16
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 0
            i32.store offset=16
            local.get 0
            local.get 4
            i32.store offset=24
          end
          local.get 9
          i32.load offset=20
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 0
          i32.store offset=20
          local.get 0
          local.get 4
          i32.store offset=24
        end
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 5
            i32.or
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 9
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 9
          local.get 5
          i32.add
          local.tee 8
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 4144
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                i32.const 0
                i32.load offset=4104
                local.tee 5
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 5
                local.get 3
                i32.or
                i32.store offset=4104
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 3
            local.get 8
            i32.store offset=12
            local.get 4
            local.get 8
            i32.store offset=8
            local.get 8
            local.get 4
            i32.store offset=12
            local.get 8
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 4
          block ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 38
            local.get 3
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 4
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 4
          end
          local.get 8
          local.get 4
          i32.store offset=28
          local.get 8
          i64.const 0
          i64.store offset=16 align=4
          local.get 4
          i32.const 2
          i32.shl
          i32.const 4408
          i32.add
          local.set 5
          block ;; label = @4
            local.get 10
            i32.const 1
            local.get 4
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            local.get 5
            local.get 8
            i32.store
            i32.const 0
            local.get 10
            local.get 0
            i32.or
            i32.store offset=4108
            local.get 8
            local.get 5
            i32.store offset=24
            local.get 8
            local.get 8
            i32.store offset=8
            local.get 8
            local.get 8
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 4
          local.get 5
          i32.load
          local.set 0
          block ;; label = @4
            loop ;; label = @5
              local.get 0
              local.tee 5
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 0
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 5
              local.get 0
              i32.const 4
              i32.and
              i32.add
              local.tee 6
              i32.load offset=16
              local.tee 0
              br_if 0 (;@5;)
            end
            local.get 6
            i32.const 16
            i32.add
            local.get 8
            i32.store
            local.get 8
            local.get 5
            i32.store offset=24
            local.get 8
            local.get 8
            i32.store offset=12
            local.get 8
            local.get 8
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 5
          i32.load offset=8
          local.tee 4
          local.get 8
          i32.store offset=12
          local.get 5
          local.get 8
          i32.store offset=8
          local.get 8
          i32.const 0
          i32.store offset=24
          local.get 8
          local.get 5
          i32.store offset=12
          local.get 8
          local.get 4
          i32.store offset=8
        end
        local.get 9
        i32.const 8
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 8
            local.get 8
            i32.load offset=28
            local.tee 9
            i32.const 2
            i32.shl
            local.tee 0
            i32.load offset=4408
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 4408
            i32.add
            local.get 4
            i32.store
            local.get 4
            br_if 1 (;@3;)
            i32.const 0
            local.get 10
            i32.const -2
            local.get 9
            i32.rotl
            i32.and
            i32.store offset=4108
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=16
              local.get 8
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 4
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 2
            local.get 4
            i32.store offset=20
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        local.get 2
        i32.store offset=24
        block ;; label = @3
          local.get 8
          i32.load offset=16
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 0
          i32.store offset=16
          local.get 0
          local.get 4
          i32.store offset=24
        end
        local.get 8
        i32.load offset=20
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.store offset=20
        local.get 0
        local.get 4
        i32.store offset=24
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 8
          local.get 3
          local.get 5
          i32.or
          local.tee 4
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 8
        local.get 5
        i32.add
        local.tee 0
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 8
        local.get 5
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.get 3
        i32.store
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          i32.const -8
          i32.and
          i32.const 4144
          i32.add
          local.set 5
          i32.const 0
          i32.load offset=4124
          local.set 4
          block ;; label = @4
            block ;; label = @5
              i32.const 1
              local.get 7
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 6
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 6
              i32.or
              i32.store offset=4104
              local.get 5
              local.set 9
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 5
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 0
        i32.store offset=4124
        i32.const 0
        local.get 3
        i32.store offset=4112
      end
      local.get 8
      i32.const 8
      i32.add
      local.set 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $prepend_alloc (;31;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    local.get 0
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 3
    local.get 2
    i32.const 3
    i32.or
    i32.store offset=4
    local.get 1
    i32.const -8
    local.get 1
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 4
    local.get 3
    local.get 2
    i32.add
    local.tee 5
    i32.sub
    local.set 0
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=4128
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=4128
        i32.const 0
        i32.const 0
        i32.load offset=4116
        local.get 0
        i32.add
        local.tee 2
        i32.store offset=4116
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=4124
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=4124
        i32.const 0
        i32.const 0
        i32.load offset=4112
        local.get 0
        i32.add
        local.tee 2
        i32.store offset=4112
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 5
        local.get 2
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 4
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.set 6
        local.get 4
        i32.load offset=12
        local.set 2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 2
              local.get 4
              i32.load offset=8
              local.tee 7
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=4104
              i32.const -2
              local.get 1
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=4104
              br 2 (;@3;)
            end
            local.get 2
            local.get 7
            i32.store offset=8
            local.get 7
            local.get 2
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 4
          i32.load offset=24
          local.set 8
          block ;; label = @4
            block ;; label = @5
              local.get 2
              local.get 4
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  i32.load offset=20
                  local.tee 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 20
                  i32.add
                  local.set 7
                  br 1 (;@6;)
                end
                local.get 4
                i32.load offset=16
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                local.get 4
                i32.const 16
                i32.add
                local.set 7
              end
              loop ;; label = @6
                local.get 7
                local.set 9
                local.get 1
                local.tee 2
                i32.const 20
                i32.add
                local.set 7
                local.get 2
                i32.load offset=20
                local.tee 1
                br_if 0 (;@6;)
                local.get 2
                i32.const 16
                i32.add
                local.set 7
                local.get 2
                i32.load offset=16
                local.tee 1
                br_if 0 (;@6;)
              end
              local.get 9
              i32.const 0
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.set 2
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 4
              local.get 4
              i32.load offset=28
              local.tee 7
              i32.const 2
              i32.shl
              local.tee 1
              i32.load offset=4408
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 4408
              i32.add
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4108
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store offset=4108
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load offset=16
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 8
              local.get 2
              i32.store offset=20
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 8
          i32.store offset=24
          block ;; label = @4
            local.get 4
            i32.load offset=16
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 2
            i32.store offset=24
          end
          local.get 4
          i32.load offset=20
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.store offset=20
          local.get 1
          local.get 2
          i32.store offset=24
        end
        local.get 6
        local.get 0
        i32.add
        local.set 0
        local.get 4
        local.get 6
        i32.add
        local.tee 4
        i32.load offset=4
        local.set 1
      end
      local.get 4
      local.get 1
      i32.const -2
      i32.and
      i32.store offset=4
      local.get 5
      local.get 0
      i32.add
      local.get 0
      i32.store
      local.get 5
      local.get 0
      i32.const 1
      i32.or
      i32.store offset=4
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 4144
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=4104
            local.tee 1
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            local.get 0
            i32.or
            i32.store offset=4104
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 5
        i32.store offset=12
        local.get 2
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 2
        i32.store offset=12
        local.get 5
        local.get 0
        i32.store offset=8
        br 1 (;@1;)
      end
      i32.const 31
      local.set 2
      block ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      end
      local.get 5
      local.get 2
      i32.store offset=28
      local.get 5
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 4408
      i32.add
      local.set 1
      block ;; label = @2
        i32.const 0
        i32.load offset=4108
        local.tee 7
        i32.const 1
        local.get 2
        i32.shl
        local.tee 4
        i32.and
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        i32.store
        i32.const 0
        local.get 7
        local.get 4
        i32.or
        i32.store offset=4108
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.const 25
      local.get 2
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 2
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 2
      local.get 1
      i32.load
      local.set 7
      block ;; label = @2
        loop ;; label = @3
          local.get 7
          local.tee 1
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 0
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.const 29
          i32.shr_u
          local.set 7
          local.get 2
          i32.const 1
          i32.shl
          local.set 2
          local.get 1
          local.get 7
          i32.const 4
          i32.and
          i32.add
          local.tee 4
          i32.load offset=16
          local.tee 7
          br_if 0 (;@3;)
        end
        local.get 4
        i32.const 16
        i32.add
        local.get 5
        i32.store
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=12
        local.get 5
        local.get 5
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=8
      local.tee 2
      local.get 5
      i32.store offset=12
      local.get 1
      local.get 5
      i32.store offset=8
      local.get 5
      i32.const 0
      i32.store offset=24
      local.get 5
      local.get 1
      i32.store offset=12
      local.get 5
      local.get 2
      i32.store offset=8
    end
    local.get 3
    i32.const 8
    i32.add
  )
  (func $free (;32;) (type 0) (param i32)
    local.get 0
    call $dlfree
  )
  (func $dlfree (;33;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 4
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=4120
        i32.lt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 0
        i32.add
        local.set 0
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.const 0
                i32.load offset=4124
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=12
                local.set 2
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 1
                  i32.load offset=8
                  local.tee 5
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=4104
                  i32.const -2
                  local.get 4
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store offset=4104
                  br 5 (;@2;)
                end
                local.get 1
                i32.load offset=24
                local.set 6
                block ;; label = @7
                  local.get 2
                  local.get 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 1
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  br 4 (;@3;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 1
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 3 (;@3;)
              end
              local.get 3
              i32.load offset=4
              local.tee 2
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 3
              local.get 2
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 0
              local.get 0
              i32.store offset=4112
              local.get 3
              local.get 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              return
            end
            local.get 2
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 2
            i32.store offset=12
            br 2 (;@2;)
          end
          i32.const 0
          local.set 2
        end
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 1
            local.get 1
            i32.load offset=28
            local.tee 5
            i32.const 2
            i32.shl
            local.tee 4
            i32.load offset=4408
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            i32.const 4408
            i32.add
            local.get 2
            i32.store
            local.get 2
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=4108
            i32.const -2
            local.get 5
            i32.rotl
            i32.and
            i32.store offset=4108
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 6
              i32.load offset=16
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.get 2
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 6
            local.get 2
            i32.store offset=20
          end
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 6
        i32.store offset=24
        block ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 2
          i32.store offset=24
        end
        local.get 1
        i32.load offset=20
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        i32.store offset=20
        local.get 4
        local.get 2
        i32.store offset=24
      end
      local.get 1
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 4
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=4128
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=4128
                  i32.const 0
                  i32.const 0
                  i32.load offset=4116
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=4116
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  i32.const 0
                  i32.load offset=4124
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=4112
                  i32.const 0
                  i32.const 0
                  i32.store offset=4124
                  return
                end
                block ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=4124
                  local.tee 6
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=4124
                  i32.const 0
                  i32.const 0
                  i32.load offset=4112
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=4112
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.add
                  local.get 0
                  i32.store
                  return
                end
                local.get 4
                i32.const -8
                i32.and
                local.get 0
                i32.add
                local.set 0
                local.get 3
                i32.load offset=12
                local.set 2
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    local.get 3
                    i32.load offset=8
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=4104
                    i32.const -2
                    local.get 4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=4104
                    br 5 (;@3;)
                  end
                  local.get 2
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  local.get 2
                  i32.store offset=12
                  br 4 (;@3;)
                end
                local.get 3
                i32.load offset=24
                local.set 8
                block ;; label = @7
                  local.get 2
                  local.get 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  br 3 (;@4;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 2 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 2
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 3
              local.get 3
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              local.tee 4
              i32.load offset=4408
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.const 4408
              i32.add
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4108
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=4108
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load offset=16
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 8
              local.get 2
              i32.store offset=20
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 8
          i32.store offset=24
          block ;; label = @4
            local.get 3
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 2
            i32.store offset=24
          end
          local.get 3
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.store offset=20
          local.get 4
          local.get 2
          i32.store offset=24
        end
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 6
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 0
        i32.store offset=4112
        return
      end
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 4144
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=4104
            local.tee 4
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 0
            i32.or
            i32.store offset=4104
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 2
      block ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 4408
      i32.add
      local.set 5
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=4108
              local.tee 4
              i32.const 1
              local.get 2
              i32.shl
              local.tee 3
              i32.and
              br_if 0 (;@5;)
              local.get 5
              local.get 1
              i32.store
              i32.const 0
              local.get 4
              local.get 3
              i32.or
              i32.store offset=4108
              i32.const 8
              local.set 0
              i32.const 24
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            i32.const 0
            i32.const 25
            local.get 2
            i32.const 1
            i32.shr_u
            i32.sub
            local.get 2
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set 2
            local.get 5
            i32.load
            local.set 5
            loop ;; label = @5
              local.get 5
              local.tee 4
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 29
              i32.shr_u
              local.set 5
              local.get 2
              i32.const 1
              i32.shl
              local.set 2
              local.get 4
              local.get 5
              i32.const 4
              i32.and
              i32.add
              local.tee 3
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 16
            i32.add
            local.get 1
            i32.store
            i32.const 8
            local.set 0
            i32.const 24
            local.set 2
            local.get 4
            local.set 5
          end
          local.get 1
          local.set 4
          local.get 1
          local.set 3
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 5
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        i32.const 0
        local.set 3
        i32.const 24
        local.set 0
        i32.const 8
        local.set 2
      end
      local.get 1
      local.get 2
      i32.add
      local.get 5
      i32.store
      local.get 1
      local.get 4
      i32.store offset=12
      local.get 1
      local.get 0
      i32.add
      local.get 3
      i32.store
      i32.const 0
      i32.const 0
      i32.load offset=4136
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=4136
    end
  )
  (func $calloc (;34;) (type 5) (param i32 i32) (result i32)
    (local i32 i64)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block ;; label = @1
      local.get 2
      call $dlmalloc
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      memory.fill
    end
    local.get 0
  )
  (func $open (;35;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.const 12
        i32.add
        i32.const 4600
        i32.const 4604
        i32.const 1
        call $undefined_weak:__wasilibc_find_relpath_alloc
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.const 12
      i32.add
      i32.const 4600
      i32.const 0
      i32.load offset=4604
      call $__wasilibc_find_relpath
      local.set 0
    end
    i32.const -1
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 44
        i32.store offset=4100
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.load offset=4600
      local.get 1
      call $__wasilibc_nocwd_openat_nomode
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $_Exit (;36;) (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $abort (;37;) (type 3)
    unreachable
  )
  (func $__wasilibc_populate_preopens (;38;) (type 3)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=4616
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=4616
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 3
      local.set 1
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 0
              i32.const 8
              i32.add
              call $__wasi_fd_prestat_get
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 8
              i32.ne
              br_if 2 (;@3;)
              i32.const 0
              i32.const 1
              i32.store8 offset=4616
              br 4 (;@1;)
            end
            block ;; label = @5
              local.get 0
              i32.load8_u offset=8
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=12
              local.tee 3
              i32.const 1
              i32.add
              call $malloc
              local.tee 2
              i32.eqz
              br_if 3 (;@2;)
              local.get 1
              local.get 2
              local.get 3
              call $__wasi_fd_prestat_dir_name
              br_if 2 (;@3;)
              local.get 2
              local.get 0
              i32.load offset=12
              i32.add
              i32.const 0
              i32.store8
              local.get 1
              local.get 2
              call $internal_register_preopened_fd_unlocked
              br_if 3 (;@2;)
              local.get 2
              call $free
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            br 0 (;@4;)
          end
        end
        i32.const 71
        call $_Exit
        unreachable
      end
      i32.const 70
      call $_Exit
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $internal_register_preopened_fd_unlocked (;39;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 2
        i32.add
        br_table 1 (;@1;) 1 (;@1;) 0 (;@2;)
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        i32.const 0
        i32.load offset=4608
        local.tee 2
        i32.const 0
        i32.load offset=4620
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=4612
        local.set 3
        block ;; label = @3
          i32.const 8
          local.get 2
          i32.const 1
          i32.shl
          i32.const 4
          local.get 2
          select
          local.tee 4
          call $calloc
          local.tee 5
          br_if 0 (;@3;)
          i32.const -1
          return
        end
        block ;; label = @3
          local.get 2
          i32.const 3
          i32.shl
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          local.get 6
          memory.copy
        end
        i32.const 0
        local.get 4
        i32.store offset=4620
        i32.const 0
        local.get 5
        i32.store offset=4612
        local.get 3
        call $free
      end
      block ;; label = @2
        loop ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              local.tee 3
              i32.load8_u
              i32.const -46
              i32.add
              br_table 1 (;@4;) 0 (;@5;) 3 (;@2;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.load8_u offset=1
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const 47
          i32.ne
          br_if 1 (;@2;)
          local.get 3
          i32.const 2
          i32.add
          local.set 1
          br 0 (;@3;)
        end
      end
      block ;; label = @2
        local.get 3
        call $strdup
        local.tee 3
        br_if 0 (;@2;)
        i32.const -1
        return
      end
      i32.const 0
      local.get 2
      i32.const 1
      i32.add
      i32.store offset=4608
      i32.const 0
      i32.load offset=4612
      local.get 2
      i32.const 3
      i32.shl
      i32.add
      local.tee 1
      local.get 0
      i32.store offset=4
      local.get 1
      local.get 3
      i32.store
      i32.const 0
      return
    end
    call $abort
    unreachable
  )
  (func $__wasilibc_find_relpath (;40;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 2
        local.get 4
        i32.const 12
        i32.add
        i32.const 0
        call $undefined_weak:__wasilibc_find_relpath_alloc
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      local.get 2
      call $__wasilibc_find_abspath
      local.set 3
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $__wasilibc_find_abspath (;41;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const -1
    i32.add
    local.set 0
    call $__wasilibc_populate_preopens
    loop ;; label = @1
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.load8_u
      i32.const 47
      i32.eq
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 3
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=4608
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=4612
        local.set 5
        i32.const -1
        local.set 6
        loop ;; label = @3
          local.get 5
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 7
          i32.const -8
          i32.add
          i32.load
          local.tee 8
          call $strlen
          local.set 9
          block ;; label = @4
            block ;; label = @5
              local.get 6
              i32.const -1
              i32.eq
              br_if 0 (;@5;)
              local.get 9
              local.get 3
              i32.le_u
              br_if 1 (;@4;)
            end
            local.get 0
            i32.load8_u
            local.set 10
            block ;; label = @5
              block ;; label = @6
                local.get 9
                br_if 0 (;@6;)
                local.get 10
                i32.const 255
                i32.and
                i32.const 47
                i32.ne
                br_if 1 (;@5;)
              end
              local.get 0
              local.get 8
              local.get 9
              call $memcmp
              br_if 1 (;@4;)
              block ;; label = @6
                local.get 9
                i32.eqz
                br_if 0 (;@6;)
                local.get 8
                i32.const -1
                i32.add
                local.set 11
                local.get 9
                local.set 10
                block ;; label = @7
                  loop ;; label = @8
                    local.get 11
                    local.get 10
                    i32.add
                    i32.load8_u
                    i32.const 47
                    i32.ne
                    br_if 1 (;@7;)
                    local.get 10
                    i32.const -1
                    i32.add
                    local.tee 10
                    br_if 0 (;@8;)
                  end
                  i32.const 0
                  local.set 10
                end
                local.get 0
                local.get 10
                i32.add
                i32.load8_u
                local.set 10
              end
              local.get 10
              i32.const 255
              i32.and
              local.tee 10
              i32.const 47
              i32.eq
              br_if 0 (;@5;)
              local.get 10
              br_if 1 (;@4;)
            end
            local.get 1
            local.get 8
            i32.store
            local.get 7
            i32.const -4
            i32.add
            i32.load
            local.set 6
            local.get 9
            local.set 3
          end
          local.get 4
          i32.const -1
          i32.add
          local.tee 4
          br_if 0 (;@3;)
        end
        local.get 6
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.const 44
      i32.store offset=4100
      i32.const -1
      return
    end
    local.get 0
    local.get 3
    i32.add
    i32.const -1
    i32.add
    local.set 0
    loop ;; label = @1
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.load8_u
      local.tee 4
      i32.const 47
      i32.eq
      br_if 0 (;@1;)
    end
    local.get 2
    local.get 0
    i32.const 1161
    local.get 4
    select
    i32.store
    local.get 6
  )
  (func $sbrk (;42;) (type 4) (param i32) (result i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=4100
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable
  )
  (func $__wasi_init_tp (;43;) (type 3)
    (local i32 i32)
    i32.const 0
    i32.const 5716
    i32.store offset=5716
    i32.const 71360
    local.set 0
    block ;; label = @1
      block ;; label = @2
        i32.const 71360
        i32.eqz
        br_if 0 (;@2;)
        i32.const 71360
        i32.const 5824
        i32.sub
        local.set 1
        br 1 (;@1;)
      end
      global.get $__stack_pointer
      local.set 1
      i32.const 71360
      i32.const 5824
      i32.sub
      i32.const 1024
      local.get 1
      i32.const 1024
      i32.gt_u
      local.tee 0
      select
      local.set 1
      i32.const 71360
      i32.const 1024
      local.get 0
      select
      local.set 0
    end
    i32.const 56
    i32.const 0
    i32.store offset=5716
    i32.const 52
    local.get 1
    i32.store offset=5716
    i32.const 48
    local.get 0
    i32.store offset=5716
    i32.const 8
    i32.const 5716
    i32.store offset=5716
    i32.const 4
    i32.const 5716
    i32.store offset=5716
    i32.const 12
    i32.const 0
    i32.load offset=4624
    i32.store offset=5716
    i32.const 0
    local.get 1
    i32.const 8388608
    local.get 1
    i32.const 8388608
    i32.lt_u
    select
    i32.store offset=4084
  )
  (func $dummy (;44;) (type 3))
  (func $__wasm_call_dtors (;45;) (type 3)
    call $dummy
    call $__stdio_exit
  )
  (func $printf (;46;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=12
    i32.const 3848
    local.get 0
    local.get 1
    call $vfprintf
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__stdio_close (;47;) (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call $close
  )
  (func $writev (;48;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const -1
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 28
        i32.store offset=4100
        br 1 (;@1;)
      end
      local.get 3
      i32.const 0
      i32.store offset=12
      block ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        i32.const 12
        i32.add
        call $__wasi_fd_write
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i32.store offset=4100
        i32.const -1
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=12
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $__stdio_write (;49;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load offset=24
    local.tee 1
    i32.store
    local.get 3
    local.get 0
    i32.load offset=20
    local.get 1
    i32.sub
    local.tee 4
    i32.store offset=4
    i32.const 2
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=56
        local.get 3
        i32.const 2
        call $writev
        local.tee 1
        local.get 4
        local.get 2
        i32.add
        local.tee 6
        i32.eq
        br_if 0 (;@2;)
        local.get 3
        local.set 4
        loop ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const -1
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            i64.const 0
            i64.store offset=16
            local.get 0
            local.get 0
            i32.load
            i32.const 32
            i32.or
            i32.store
            local.get 5
            i32.const 2
            i32.eq
            br_if 3 (;@1;)
            local.get 2
            local.get 4
            i32.load offset=4
            i32.sub
            local.set 1
            br 3 (;@1;)
          end
          local.get 4
          i32.const 8
          i32.const 0
          local.get 1
          local.get 4
          i32.load offset=4
          local.tee 7
          i32.gt_u
          local.tee 8
          select
          i32.add
          local.tee 9
          local.get 9
          i32.load
          local.get 1
          local.get 7
          i32.const 0
          local.get 8
          select
          i32.sub
          local.tee 7
          i32.add
          i32.store
          local.get 4
          i32.const 12
          i32.const 4
          local.get 8
          select
          i32.add
          local.tee 4
          local.get 4
          i32.load
          local.get 7
          i32.sub
          i32.store
          local.get 9
          local.set 4
          local.get 6
          local.get 1
          i32.sub
          local.tee 6
          local.get 0
          i32.load offset=56
          local.get 9
          local.get 5
          local.get 8
          i32.sub
          local.tee 5
          call $writev
          local.tee 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 0
      i32.load offset=40
      local.tee 1
      i32.store offset=24
      local.get 0
      local.get 1
      i32.store offset=20
      local.get 0
      local.get 1
      local.get 0
      i32.load offset=44
      i32.add
      i32.store offset=16
      local.get 2
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__isatty (;50;) (type 4) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 1
        i32.const 8
        i32.add
        call $__wasi_fd_fdstat_get
        local.tee 0
        br_if 0 (;@2;)
        i32.const 59
        local.set 0
        local.get 1
        i32.load8_u offset=8
        i32.const 2
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=16
        i32.const 36
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 2
      i32.const 0
      local.get 0
      i32.store offset=4100
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $__stdout_write (;51;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 2
    i32.store offset=32
    block ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 64
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=56
      call $__isatty
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.store offset=64
    end
    local.get 0
    local.get 1
    local.get 2
    call $__stdio_write
  )
  (func $__lseek (;52;) (type 2) (param i32 i64 i32) (result i64)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        i32.const 255
        i32.and
        local.get 3
        i32.const 8
        i32.add
        call $__wasi_fd_seek
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.const 70
        local.get 2
        local.get 2
        i32.const 76
        i32.eq
        select
        i32.store offset=4100
        i64.const -1
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      i64.load offset=8
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__stdio_seek (;53;) (type 2) (param i32 i64 i32) (result i64)
    local.get 0
    i32.load offset=56
    local.get 1
    local.get 2
    call $__lseek
  )
  (func $__ofl_lock (;54;) (type 10) (result i32)
    i32.const 5672
  )
  (func $__stdio_exit (;55;) (type 3)
    (local i32 i32 i32)
    block ;; label = @1
      call $__ofl_lock
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      loop ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=24
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 1)
          drop
        end
        block ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 1
          local.get 0
          i32.load offset=8
          local.tee 2
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          local.get 2
          i32.sub
          i64.extend_i32_s
          i32.const 1
          local.get 0
          i32.load offset=36
          call_indirect (type 2)
          drop
        end
        local.get 0
        i32.load offset=52
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      i32.const 0
      i32.load offset=5676
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 1)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 2)
      drop
    end
    block ;; label = @1
      i32.const 0
      i32.load offset=3960
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 1)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 2)
      drop
    end
    block ;; label = @1
      i32.const 0
      i32.load offset=4080
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 1)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 2)
      drop
    end
  )
  (func $__towrite (;56;) (type 4) (param i32) (result i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load offset=60
    local.tee 1
    i32.const -1
    i32.add
    local.get 1
    i32.or
    i32.store offset=60
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 8
      i32.and
      i32.eqz
      br_if 0 (;@1;)
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
    i32.load offset=40
    local.tee 1
    i32.store offset=24
    local.get 0
    local.get 1
    i32.store offset=20
    local.get 0
    local.get 1
    local.get 0
    i32.load offset=44
    i32.add
    i32.store offset=16
    i32.const 0
  )
  (func $__fwritex (;57;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load offset=16
        local.tee 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        local.get 2
        call $__towrite
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=16
        local.set 3
      end
      block ;; label = @2
        local.get 1
        local.get 3
        local.get 2
        i32.load offset=20
        local.tee 5
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        local.get 1
        local.get 2
        i32.load offset=32
        call_indirect (type 1)
        return
      end
      i32.const 0
      local.set 6
      block ;; label = @2
        local.get 2
        i32.load offset=64
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.add
        local.set 4
        i32.const 0
        local.set 3
        block ;; label = @3
          loop ;; label = @4
            local.get 4
            local.get 3
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            br_if 1 (;@3;)
            local.get 1
            local.get 3
            i32.const -1
            i32.add
            local.tee 3
            i32.add
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 6
          br 1 (;@2;)
        end
        local.get 2
        local.get 0
        local.get 1
        local.get 3
        i32.add
        local.tee 6
        local.get 2
        i32.load offset=32
        call_indirect (type 1)
        local.tee 4
        local.get 6
        i32.lt_u
        br_if 1 (;@1;)
        local.get 6
        local.get 0
        i32.add
        local.set 0
        i32.const 0
        local.get 3
        i32.sub
        local.set 1
        local.get 2
        i32.load offset=20
        local.set 5
      end
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 0
        local.get 1
        memory.copy
      end
      local.get 2
      local.get 2
      i32.load offset=20
      local.get 1
      i32.add
      i32.store offset=20
      local.get 6
      local.get 1
      i32.add
      local.set 4
    end
    local.get 4
  )
  (func $fwrite (;58;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 2
    local.get 1
    i32.mul
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.load offset=16
        local.tee 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 6
        local.get 3
        call $__towrite
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=16
        local.set 5
      end
      block ;; label = @2
        local.get 4
        local.get 5
        local.get 3
        i32.load offset=20
        local.tee 7
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        local.get 4
        local.get 3
        i32.load offset=32
        call_indirect (type 1)
        local.set 6
        br 1 (;@1;)
      end
      i32.const 0
      local.set 8
      block ;; label = @2
        block ;; label = @3
          local.get 4
          br_if 0 (;@3;)
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        i32.const 0
        local.set 5
        block ;; label = @3
          local.get 3
          i32.load offset=64
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        local.get 0
        local.get 4
        i32.add
        local.set 6
        block ;; label = @3
          loop ;; label = @4
            local.get 6
            local.get 5
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            br_if 1 (;@3;)
            local.get 4
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            i32.add
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 8
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        local.get 3
        local.get 0
        local.get 4
        local.get 5
        i32.add
        local.tee 8
        local.get 3
        i32.load offset=32
        call_indirect (type 1)
        local.tee 6
        local.get 8
        i32.lt_u
        br_if 1 (;@1;)
        local.get 8
        local.get 0
        i32.add
        local.set 0
        i32.const 0
        local.get 5
        i32.sub
        local.set 5
        local.get 3
        i32.load offset=20
        local.set 7
      end
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 0
        local.get 5
        memory.copy
      end
      local.get 3
      local.get 3
      i32.load offset=20
      local.get 5
      i32.add
      i32.store offset=20
      local.get 8
      local.get 5
      i32.add
      local.set 6
    end
    block ;; label = @1
      local.get 6
      local.get 4
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      i32.const 0
      local.get 1
      select
      return
    end
    local.get 6
    local.get 1
    i32.div_u
  )
  (func $"#func59 dummy" (@name "dummy") (;59;) (type 5) (param i32 i32) (result i32)
    local.get 0
  )
  (func $__lctrans (;60;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $"#func59 dummy"
  )
  (func $strerror (;61;) (type 4) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 0
      i32.load offset=5704
      local.tee 1
      br_if 0 (;@1;)
      i32.const 5680
      local.set 1
      i32.const 0
      i32.const 5680
      i32.store offset=5704
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.load16_u offset=2928
    i32.const 1370
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans
  )
  (func $wcrtomb (;62;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    i32.const 1
    local.set 3
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 1
        i32.const 127
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.store8
        i32.const 1
        return
      end
      block ;; label = @2
        i32.const 0
        i32.load offset=5704
        local.tee 3
        br_if 0 (;@2;)
        i32.const 5680
        local.set 3
        i32.const 0
        i32.const 5680
        i32.store offset=5704
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 1
            i32.const -128
            i32.and
            i32.const 57216
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i32.const 25
            i32.store offset=4100
            br 2 (;@2;)
          end
          local.get 0
          local.get 1
          i32.store8
          i32.const 1
          return
        end
        block ;; label = @3
          local.get 1
          i32.const 2047
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8
          i32.const 2
          return
        end
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 55296
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const -8192
            i32.and
            i32.const 57344
            i32.ne
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=2
          local.get 0
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          i32.const 3
          return
        end
        block ;; label = @3
          local.get 1
          i32.const -65536
          i32.add
          i32.const 1048575
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=3
          local.get 0
          local.get 1
          i32.const 18
          i32.shr_u
          i32.const 240
          i32.or
          i32.store8
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=2
          local.get 0
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          i32.const 4
          return
        end
        i32.const 0
        i32.const 25
        i32.store offset=4100
      end
      i32.const -1
      local.set 3
    end
    local.get 3
  )
  (func $wctomb (;63;) (type 5) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 0
    local.get 1
    i32.const 0
    call $wcrtomb
  )
  (func $frexp (;64;) (type 12) (param f64 i32) (result f64)
    (local i64 i32)
    block ;; label = @1
      local.get 0
      i64.reinterpret_f64
      local.tee 2
      i64.const 52
      i64.shr_u
      i32.wrap_i64
      i32.const 2047
      i32.and
      local.tee 3
      i32.const 2047
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          f64.const 0x0p+0 (;=0;)
          f64.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 0
          i32.store
          local.get 0
          return
        end
        local.get 0
        f64.const 0x1p+64 (;=18446744073709552000;)
        f64.mul
        local.get 1
        call $frexp
        local.set 0
        local.get 1
        local.get 1
        i32.load
        i32.const -64
        i32.add
        i32.store
        local.get 0
        return
      end
      local.get 1
      local.get 3
      i32.const -1022
      i32.add
      i32.store
      local.get 2
      i64.const -9218868437227405313
      i64.and
      i64.const 4602678819172646912
      i64.or
      f64.reinterpret_i64
      local.set 0
    end
    local.get 0
  )
  (func $fputs (;65;) (type 5) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    call $strlen
    local.set 2
    i32.const -1
    i32.const 0
    local.get 2
    local.get 0
    i32.const 1
    local.get 2
    local.get 1
    call $fwrite
    i32.ne
    select
  )
  (func $vfprintf (;66;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 208
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=204
    local.get 3
    i32.const 160
    i32.add
    i32.const 32
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i32.const 184
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i32.const 176
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i64.const 0
    i64.store offset=168
    local.get 3
    i64.const 0
    i64.store offset=160
    local.get 3
    local.get 2
    i32.store offset=200
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        local.get 1
        local.get 3
        i32.const 200
        i32.add
        local.get 3
        i32.const 80
        i32.add
        local.get 3
        i32.const 160
        i32.add
        call $printf_core
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const -1
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 0
      i32.load
      local.tee 4
      i32.const -33
      i32.and
      i32.store
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load offset=44
              br_if 0 (;@5;)
              local.get 0
              i32.const 80
              i32.store offset=44
              local.get 0
              i32.const 0
              i32.store offset=24
              local.get 0
              i64.const 0
              i64.store offset=16
              local.get 0
              i32.load offset=40
              local.set 5
              local.get 0
              local.get 3
              i32.store offset=40
              br 1 (;@4;)
            end
            i32.const 0
            local.set 5
            local.get 0
            i32.load offset=16
            br_if 1 (;@3;)
          end
          i32.const -1
          local.set 2
          local.get 0
          call $__towrite
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        local.get 3
        i32.const 200
        i32.add
        local.get 3
        i32.const 80
        i32.add
        local.get 3
        i32.const 160
        i32.add
        call $printf_core
        local.set 2
      end
      local.get 4
      i32.const 32
      i32.and
      local.set 1
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 1)
        drop
        local.get 0
        i32.const 0
        i32.store offset=44
        local.get 0
        local.get 5
        i32.store offset=40
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        i32.load offset=20
        local.set 5
        local.get 0
        i64.const 0
        i64.store offset=16
        local.get 2
        i32.const -1
        local.get 5
        select
        local.set 2
      end
      local.get 0
      local.get 0
      i32.load
      local.tee 5
      local.get 1
      i32.or
      i32.store
      i32.const -1
      local.get 2
      local.get 5
      i32.const 32
      i32.and
      select
      local.set 0
    end
    local.get 3
    i32.const 208
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $printf_core (;67;) (type 9) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 f64 i32 i32 i32 i32 i32 i64 i32 i32 f64)
    global.get $__stack_pointer
    i32.const 864
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    i32.const 52
    i32.add
    i32.const 12
    i32.add
    local.set 6
    local.get 5
    i32.const 96
    i32.add
    i32.const -4
    i32.add
    local.set 7
    local.get 5
    i32.const 16
    i32.add
    i32.const 25
    i32.add
    local.set 8
    local.get 5
    i32.const 39
    i32.add
    local.set 9
    local.get 5
    i32.const 52
    i32.add
    i32.const 11
    i32.add
    local.set 10
    local.get 5
    i32.const 64
    i32.add
    i32.const -1
    i32.add
    local.set 11
    local.get 5
    i32.const 64
    i32.add
    i32.const 8
    i32.or
    local.set 12
    local.get 5
    i32.const 64
    i32.add
    i32.const 9
    i32.or
    local.set 13
    local.get 5
    i32.const 52
    i32.add
    i32.const 10
    i32.add
    local.set 14
    local.get 5
    i32.const 40
    i32.add
    local.set 15
    i32.const 0
    local.set 16
    i32.const 0
    local.set 17
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          i32.const 0
          local.set 18
          block ;; label = @4
            loop ;; label = @5
              local.get 1
              local.set 19
              local.get 18
              local.get 17
              i32.const 2147483647
              i32.xor
              i32.gt_s
              br_if 1 (;@4;)
              local.get 18
              local.get 17
              i32.add
              local.set 17
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 19
                                i32.load8_u
                                local.tee 18
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 19
                                local.set 1
                                loop ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        local.get 18
                                        i32.const 255
                                        i32.and
                                        local.tee 18
                                        i32.eqz
                                        br_if 0 (;@18;)
                                        local.get 18
                                        i32.const 37
                                        i32.ne
                                        br_if 2 (;@16;)
                                        local.get 1
                                        local.set 18
                                        loop ;; label = @19
                                          local.get 1
                                          i32.const 1
                                          i32.add
                                          i32.load8_u
                                          i32.const 37
                                          i32.ne
                                          br_if 2 (;@17;)
                                          local.get 18
                                          i32.const 1
                                          i32.add
                                          local.set 18
                                          local.get 1
                                          i32.const 2
                                          i32.add
                                          local.tee 1
                                          i32.load8_u
                                          i32.const 37
                                          i32.eq
                                          br_if 0 (;@19;)
                                          br 2 (;@17;)
                                        end
                                      end
                                      local.get 1
                                      local.set 18
                                    end
                                    local.get 18
                                    local.get 19
                                    i32.sub
                                    local.tee 18
                                    local.get 17
                                    i32.const 2147483647
                                    i32.xor
                                    local.tee 20
                                    i32.gt_s
                                    br_if 12 (;@4;)
                                    block ;; label = @17
                                      local.get 0
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 0
                                      i32.load8_u
                                      i32.const 32
                                      i32.and
                                      br_if 0 (;@17;)
                                      local.get 19
                                      local.get 18
                                      local.get 0
                                      call $__fwritex
                                      drop
                                    end
                                    local.get 18
                                    br_if 11 (;@5;)
                                    local.get 1
                                    i32.const 1
                                    i32.add
                                    local.set 21
                                    i32.const -1
                                    local.set 22
                                    block ;; label = @17
                                      local.get 1
                                      i32.load8_s offset=1
                                      local.tee 23
                                      i32.const -48
                                      i32.add
                                      local.tee 18
                                      i32.const 9
                                      i32.gt_u
                                      br_if 0 (;@17;)
                                      local.get 1
                                      i32.load8_u offset=2
                                      i32.const 36
                                      i32.ne
                                      br_if 0 (;@17;)
                                      local.get 1
                                      i32.const 3
                                      i32.add
                                      local.set 21
                                      local.get 1
                                      i32.load8_s offset=3
                                      local.set 23
                                      i32.const 1
                                      local.set 16
                                      local.get 18
                                      local.set 22
                                    end
                                    i32.const 0
                                    local.set 24
                                    block ;; label = @17
                                      block ;; label = @18
                                        local.get 23
                                        i32.const -32
                                        i32.add
                                        local.tee 1
                                        i32.const 31
                                        i32.le_u
                                        br_if 0 (;@18;)
                                        local.get 21
                                        local.set 1
                                        br 1 (;@17;)
                                      end
                                      block ;; label = @18
                                        i32.const 1
                                        local.get 1
                                        i32.shl
                                        local.tee 18
                                        i32.const 75913
                                        i32.and
                                        br_if 0 (;@18;)
                                        local.get 21
                                        local.set 1
                                        br 1 (;@17;)
                                      end
                                      local.get 21
                                      i32.const 1
                                      i32.add
                                      local.set 21
                                      i32.const 0
                                      local.set 24
                                      loop ;; label = @18
                                        local.get 18
                                        local.get 24
                                        i32.or
                                        local.set 24
                                        local.get 21
                                        local.tee 1
                                        i32.load8_s
                                        local.tee 23
                                        i32.const -32
                                        i32.add
                                        local.tee 18
                                        i32.const 32
                                        i32.ge_u
                                        br_if 1 (;@17;)
                                        local.get 1
                                        i32.const 1
                                        i32.add
                                        local.set 21
                                        i32.const 1
                                        local.get 18
                                        i32.shl
                                        local.tee 18
                                        i32.const 75913
                                        i32.and
                                        br_if 0 (;@18;)
                                      end
                                    end
                                    block ;; label = @17
                                      local.get 23
                                      i32.const 42
                                      i32.ne
                                      br_if 0 (;@17;)
                                      block ;; label = @18
                                        block ;; label = @19
                                          local.get 1
                                          i32.load8_s offset=1
                                          i32.const -48
                                          i32.add
                                          local.tee 18
                                          i32.const 9
                                          i32.gt_u
                                          br_if 0 (;@19;)
                                          local.get 1
                                          i32.load8_u offset=2
                                          i32.const 36
                                          i32.ne
                                          br_if 0 (;@19;)
                                          block ;; label = @20
                                            block ;; label = @21
                                              local.get 0
                                              br_if 0 (;@21;)
                                              local.get 4
                                              local.get 18
                                              i32.const 2
                                              i32.shl
                                              i32.add
                                              i32.const 10
                                              i32.store
                                              i32.const 0
                                              local.set 25
                                              br 1 (;@20;)
                                            end
                                            local.get 3
                                            local.get 18
                                            i32.const 3
                                            i32.shl
                                            i32.add
                                            i32.load
                                            local.set 25
                                          end
                                          local.get 1
                                          i32.const 3
                                          i32.add
                                          local.set 1
                                          i32.const 1
                                          local.set 16
                                          br 1 (;@18;)
                                        end
                                        local.get 16
                                        br_if 6 (;@12;)
                                        local.get 1
                                        i32.const 1
                                        i32.add
                                        local.set 1
                                        block ;; label = @19
                                          local.get 0
                                          br_if 0 (;@19;)
                                          i32.const 0
                                          local.set 16
                                          i32.const 0
                                          local.set 25
                                          br 6 (;@13;)
                                        end
                                        local.get 2
                                        local.get 2
                                        i32.load
                                        local.tee 18
                                        i32.const 4
                                        i32.add
                                        i32.store
                                        local.get 18
                                        i32.load
                                        local.set 25
                                        i32.const 0
                                        local.set 16
                                      end
                                      local.get 25
                                      i32.const -1
                                      i32.gt_s
                                      br_if 4 (;@13;)
                                      i32.const 0
                                      local.get 25
                                      i32.sub
                                      local.set 25
                                      local.get 24
                                      i32.const 8192
                                      i32.or
                                      local.set 24
                                      br 4 (;@13;)
                                    end
                                    i32.const 0
                                    local.set 25
                                    local.get 23
                                    i32.const -48
                                    i32.add
                                    local.tee 21
                                    i32.const 9
                                    i32.gt_u
                                    br_if 3 (;@13;)
                                    local.get 1
                                    local.set 18
                                    loop ;; label = @17
                                      block ;; label = @18
                                        local.get 25
                                        i32.const 214748364
                                        i32.gt_u
                                        br_if 0 (;@18;)
                                        i32.const -1
                                        local.get 25
                                        i32.const 10
                                        i32.mul
                                        local.tee 1
                                        local.get 21
                                        i32.add
                                        local.get 21
                                        local.get 1
                                        i32.const 2147483647
                                        i32.xor
                                        i32.gt_u
                                        local.tee 23
                                        select
                                        local.set 25
                                        local.get 18
                                        i32.load8_s offset=1
                                        local.set 21
                                        local.get 18
                                        i32.const 1
                                        i32.add
                                        local.tee 1
                                        local.set 18
                                        local.get 21
                                        i32.const -48
                                        i32.add
                                        local.tee 21
                                        i32.const 10
                                        i32.lt_u
                                        br_if 1 (;@17;)
                                        local.get 23
                                        br_if 14 (;@4;)
                                        br 5 (;@13;)
                                      end
                                      local.get 18
                                      i32.load8_s offset=1
                                      local.set 1
                                      i32.const -1
                                      local.set 25
                                      local.get 18
                                      i32.const 1
                                      i32.add
                                      local.set 18
                                      local.get 1
                                      i32.const -48
                                      i32.add
                                      local.tee 21
                                      i32.const 10
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      br 13 (;@4;)
                                    end
                                  end
                                  local.get 1
                                  i32.const 1
                                  i32.add
                                  local.tee 1
                                  i32.load8_u
                                  local.set 18
                                  br 0 (;@15;)
                                end
                              end
                              local.get 0
                              br_if 12 (;@1;)
                              block ;; label = @14
                                local.get 16
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 17
                                br 13 (;@1;)
                              end
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=4
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 1
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 8
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=8
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 2
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 16
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=12
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 3
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 24
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=16
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 4
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 32
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=20
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 5
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 40
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=24
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 6
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 48
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=28
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 7
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 56
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=32
                                    local.tee 1
                                    br_if 0 (;@16;)
                                    i32.const 8
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 64
                                  i32.add
                                  local.get 1
                                  local.get 2
                                  call $pop_arg
                                  local.get 4
                                  i32.load offset=36
                                  local.tee 1
                                  br_if 1 (;@14;)
                                  i32.const 9
                                  local.set 1
                                end
                                local.get 1
                                i32.const 2
                                i32.shl
                                local.set 1
                                loop ;; label = @15
                                  local.get 4
                                  local.get 1
                                  i32.add
                                  i32.load
                                  br_if 3 (;@12;)
                                  local.get 1
                                  i32.const 4
                                  i32.add
                                  local.tee 1
                                  i32.const 40
                                  i32.ne
                                  br_if 0 (;@15;)
                                end
                                i32.const 1
                                local.set 17
                                br 13 (;@1;)
                              end
                              local.get 3
                              i32.const 72
                              i32.add
                              local.get 1
                              local.get 2
                              call $pop_arg
                              i32.const 1
                              local.set 17
                              br 12 (;@1;)
                            end
                            i32.const 0
                            local.set 18
                            block ;; label = @13
                              block ;; label = @14
                                local.get 1
                                i32.load8_u
                                i32.const 46
                                i32.eq
                                br_if 0 (;@14;)
                                i32.const -1
                                local.set 23
                                i32.const 0
                                local.set 26
                                br 1 (;@13;)
                              end
                              block ;; label = @14
                                local.get 1
                                i32.load8_s offset=1
                                local.tee 21
                                i32.const 42
                                i32.ne
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  local.get 1
                                  i32.load8_s offset=2
                                  i32.const -48
                                  i32.add
                                  local.tee 21
                                  i32.const 9
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                  local.get 1
                                  i32.load8_u offset=3
                                  i32.const 36
                                  i32.ne
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    local.get 0
                                    br_if 0 (;@16;)
                                    local.get 4
                                    local.get 21
                                    i32.const 2
                                    i32.shl
                                    i32.add
                                    i32.const 10
                                    i32.store
                                    i32.const 0
                                    local.set 23
                                    local.get 1
                                    i32.const 4
                                    i32.add
                                    local.set 1
                                    i32.const 0
                                    i32.const -1
                                    i32.gt_s
                                    local.set 26
                                    br 3 (;@13;)
                                  end
                                  local.get 1
                                  i32.const 4
                                  i32.add
                                  local.set 1
                                  local.get 3
                                  local.get 21
                                  i32.const 3
                                  i32.shl
                                  i32.add
                                  i32.load
                                  local.tee 23
                                  i32.const -1
                                  i32.gt_s
                                  local.set 26
                                  br 2 (;@13;)
                                end
                                local.get 16
                                br_if 2 (;@12;)
                                local.get 1
                                i32.const 2
                                i32.add
                                local.set 1
                                block ;; label = @15
                                  local.get 0
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 23
                                  i32.const 0
                                  i32.const -1
                                  i32.gt_s
                                  local.set 26
                                  br 2 (;@13;)
                                end
                                local.get 2
                                local.get 2
                                i32.load
                                local.tee 21
                                i32.const 4
                                i32.add
                                i32.store
                                local.get 21
                                i32.load
                                local.tee 23
                                i32.const -1
                                i32.gt_s
                                local.set 26
                                br 1 (;@13;)
                              end
                              local.get 1
                              i32.const 1
                              i32.add
                              local.set 1
                              block ;; label = @14
                                local.get 21
                                i32.const -48
                                i32.add
                                local.tee 27
                                i32.const 9
                                i32.le_u
                                br_if 0 (;@14;)
                                i32.const 1
                                local.set 26
                                i32.const 0
                                local.set 23
                                br 1 (;@13;)
                              end
                              i32.const 0
                              local.set 21
                              loop ;; label = @14
                                i32.const -1
                                local.set 23
                                block ;; label = @15
                                  local.get 21
                                  i32.const 214748364
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                  i32.const -1
                                  local.get 21
                                  i32.const 10
                                  i32.mul
                                  local.tee 21
                                  local.get 27
                                  i32.add
                                  local.get 27
                                  local.get 21
                                  i32.const 2147483647
                                  i32.xor
                                  i32.gt_u
                                  select
                                  local.set 23
                                end
                                i32.const 1
                                local.set 26
                                local.get 23
                                local.set 21
                                local.get 1
                                i32.const 1
                                i32.add
                                local.tee 1
                                i32.load8_s
                                i32.const -48
                                i32.add
                                local.tee 27
                                i32.const 10
                                i32.lt_u
                                br_if 0 (;@14;)
                              end
                            end
                            loop ;; label = @13
                              local.get 18
                              local.set 21
                              local.get 1
                              i32.load8_s
                              local.tee 18
                              i32.const -123
                              i32.add
                              i32.const -58
                              i32.lt_u
                              br_if 1 (;@12;)
                              local.get 1
                              i32.const 1
                              i32.add
                              local.set 1
                              local.get 18
                              local.get 21
                              i32.const 58
                              i32.mul
                              i32.add
                              i32.const 3023
                              i32.add
                              i32.load8_u
                              local.tee 18
                              i32.const -1
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 8
                              i32.lt_u
                              br_if 0 (;@13;)
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 18
                                i32.const 27
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 18
                                i32.eqz
                                br_if 2 (;@12;)
                                block ;; label = @15
                                  local.get 22
                                  i32.const 0
                                  i32.lt_s
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    local.get 0
                                    br_if 0 (;@16;)
                                    local.get 4
                                    local.get 22
                                    i32.const 2
                                    i32.shl
                                    i32.add
                                    local.get 18
                                    i32.store
                                    br 13 (;@3;)
                                  end
                                  local.get 5
                                  local.get 3
                                  local.get 22
                                  i32.const 3
                                  i32.shl
                                  i32.add
                                  i64.load
                                  i64.store offset=40
                                  br 2 (;@13;)
                                end
                                block ;; label = @15
                                  local.get 0
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 17
                                  br 14 (;@1;)
                                end
                                local.get 5
                                i32.const 40
                                i32.add
                                local.get 18
                                local.get 2
                                call $pop_arg
                                br 1 (;@13;)
                              end
                              local.get 22
                              i32.const -1
                              i32.gt_s
                              br_if 1 (;@12;)
                              i32.const 0
                              local.set 18
                              local.get 0
                              i32.eqz
                              br_if 8 (;@5;)
                            end
                            local.get 0
                            i32.load
                            local.tee 22
                            i32.const 32
                            i32.and
                            br_if 10 (;@2;)
                            local.get 24
                            i32.const -65537
                            i32.and
                            local.tee 27
                            local.get 24
                            local.get 24
                            i32.const 8192
                            i32.and
                            select
                            local.set 28
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        block ;; label = @19
                                          block ;; label = @20
                                            block ;; label = @21
                                              block ;; label = @22
                                                block ;; label = @23
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      block ;; label = @26
                                                        block ;; label = @27
                                                          block ;; label = @28
                                                            block ;; label = @29
                                                              block ;; label = @30
                                                                local.get 1
                                                                i32.const -1
                                                                i32.add
                                                                i32.load8_u
                                                                local.tee 24
                                                                i32.extend8_s
                                                                local.tee 18
                                                                i32.const -45
                                                                i32.and
                                                                local.get 18
                                                                local.get 24
                                                                i32.const 15
                                                                i32.and
                                                                i32.const 3
                                                                i32.eq
                                                                select
                                                                local.get 18
                                                                local.get 21
                                                                select
                                                                local.tee 29
                                                                i32.const -65
                                                                i32.add
                                                                br_table 17 (;@13;) 19 (;@11;) 12 (;@18;) 19 (;@11;) 17 (;@13;) 17 (;@13;) 17 (;@13;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 13 (;@17;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 3 (;@27;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 17 (;@13;) 19 (;@11;) 8 (;@22;) 5 (;@25;) 17 (;@13;) 17 (;@13;) 17 (;@13;) 19 (;@11;) 5 (;@25;) 19 (;@11;) 19 (;@11;) 19 (;@11;) 9 (;@21;) 1 (;@29;) 4 (;@26;) 2 (;@28;) 19 (;@11;) 19 (;@11;) 10 (;@20;) 19 (;@11;) 0 (;@30;) 19 (;@11;) 19 (;@11;) 3 (;@27;) 19 (;@11;)
                                                              end
                                                              i32.const 0
                                                              local.set 22
                                                              i32.const 1024
                                                              local.set 30
                                                              local.get 5
                                                              i64.load offset=40
                                                              local.set 31
                                                              br 5 (;@24;)
                                                            end
                                                            i32.const 0
                                                            local.set 18
                                                            block ;; label = @29
                                                              block ;; label = @30
                                                                block ;; label = @31
                                                                  block ;; label = @32
                                                                    block ;; label = @33
                                                                      block ;; label = @34
                                                                        block ;; label = @35
                                                                          local.get 21
                                                                          br_table 0 (;@35;) 1 (;@34;) 2 (;@33;) 3 (;@32;) 4 (;@31;) 30 (;@5;) 5 (;@30;) 6 (;@29;) 30 (;@5;)
                                                                        end
                                                                        local.get 5
                                                                        i32.load offset=40
                                                                        local.get 17
                                                                        i32.store
                                                                        br 29 (;@5;)
                                                                      end
                                                                      local.get 5
                                                                      i32.load offset=40
                                                                      local.get 17
                                                                      i32.store
                                                                      br 28 (;@5;)
                                                                    end
                                                                    local.get 5
                                                                    i32.load offset=40
                                                                    local.get 17
                                                                    i64.extend_i32_s
                                                                    i64.store
                                                                    br 27 (;@5;)
                                                                  end
                                                                  local.get 5
                                                                  i32.load offset=40
                                                                  local.get 17
                                                                  i32.store16
                                                                  br 26 (;@5;)
                                                                end
                                                                local.get 5
                                                                i32.load offset=40
                                                                local.get 17
                                                                i32.store8
                                                                br 25 (;@5;)
                                                              end
                                                              local.get 5
                                                              i32.load offset=40
                                                              local.get 17
                                                              i32.store
                                                              br 24 (;@5;)
                                                            end
                                                            local.get 5
                                                            i32.load offset=40
                                                            local.get 17
                                                            i64.extend_i32_s
                                                            i64.store
                                                            br 23 (;@5;)
                                                          end
                                                          local.get 23
                                                          i32.const 8
                                                          local.get 23
                                                          i32.const 8
                                                          i32.gt_u
                                                          select
                                                          local.set 23
                                                          local.get 28
                                                          i32.const 8
                                                          i32.or
                                                          local.set 28
                                                          i32.const 120
                                                          local.set 29
                                                        end
                                                        i32.const 0
                                                        local.set 22
                                                        i32.const 1024
                                                        local.set 30
                                                        block ;; label = @27
                                                          local.get 5
                                                          i64.load offset=40
                                                          local.tee 31
                                                          i64.eqz
                                                          i32.eqz
                                                          br_if 0 (;@27;)
                                                          local.get 15
                                                          local.set 19
                                                          br 4 (;@23;)
                                                        end
                                                        local.get 29
                                                        i32.const 32
                                                        i32.and
                                                        local.set 21
                                                        local.get 15
                                                        local.set 19
                                                        loop ;; label = @27
                                                          local.get 19
                                                          i32.const -1
                                                          i32.add
                                                          local.tee 19
                                                          local.get 31
                                                          i32.wrap_i64
                                                          i32.const 15
                                                          i32.and
                                                          i32.load8_u offset=3552
                                                          local.get 21
                                                          i32.or
                                                          i32.store8
                                                          local.get 31
                                                          i64.const 15
                                                          i64.gt_u
                                                          local.set 18
                                                          local.get 31
                                                          i64.const 4
                                                          i64.shr_u
                                                          local.set 31
                                                          local.get 18
                                                          br_if 0 (;@27;)
                                                        end
                                                        local.get 28
                                                        i32.const 8
                                                        i32.and
                                                        i32.eqz
                                                        br_if 3 (;@23;)
                                                        local.get 29
                                                        i32.const 4
                                                        i32.shr_u
                                                        i32.const 1024
                                                        i32.add
                                                        local.set 30
                                                        i32.const 2
                                                        local.set 22
                                                        br 3 (;@23;)
                                                      end
                                                      local.get 15
                                                      local.set 19
                                                      block ;; label = @26
                                                        local.get 5
                                                        i64.load offset=40
                                                        local.tee 31
                                                        i64.eqz
                                                        br_if 0 (;@26;)
                                                        local.get 15
                                                        local.set 19
                                                        loop ;; label = @27
                                                          local.get 19
                                                          i32.const -1
                                                          i32.add
                                                          local.tee 19
                                                          local.get 31
                                                          i32.wrap_i64
                                                          i32.const 7
                                                          i32.and
                                                          i32.const 48
                                                          i32.or
                                                          i32.store8
                                                          local.get 31
                                                          i64.const 7
                                                          i64.gt_u
                                                          local.set 18
                                                          local.get 31
                                                          i64.const 3
                                                          i64.shr_u
                                                          local.set 31
                                                          local.get 18
                                                          br_if 0 (;@27;)
                                                        end
                                                      end
                                                      i32.const 0
                                                      local.set 22
                                                      i32.const 1024
                                                      local.set 30
                                                      local.get 28
                                                      i32.const 8
                                                      i32.and
                                                      i32.eqz
                                                      br_if 2 (;@23;)
                                                      local.get 23
                                                      local.get 8
                                                      local.get 19
                                                      i32.sub
                                                      local.tee 18
                                                      local.get 23
                                                      local.get 18
                                                      i32.gt_s
                                                      select
                                                      local.set 23
                                                      br 2 (;@23;)
                                                    end
                                                    block ;; label = @25
                                                      local.get 5
                                                      i64.load offset=40
                                                      local.tee 31
                                                      i64.const -1
                                                      i64.gt_s
                                                      br_if 0 (;@25;)
                                                      local.get 5
                                                      i64.const 0
                                                      local.get 31
                                                      i64.sub
                                                      local.tee 31
                                                      i64.store offset=40
                                                      i32.const 1
                                                      local.set 22
                                                      i32.const 1024
                                                      local.set 30
                                                      br 1 (;@24;)
                                                    end
                                                    block ;; label = @25
                                                      local.get 28
                                                      i32.const 2048
                                                      i32.and
                                                      i32.eqz
                                                      br_if 0 (;@25;)
                                                      i32.const 1
                                                      local.set 22
                                                      i32.const 1025
                                                      local.set 30
                                                      br 1 (;@24;)
                                                    end
                                                    i32.const 1026
                                                    i32.const 1024
                                                    local.get 28
                                                    i32.const 1
                                                    i32.and
                                                    local.tee 22
                                                    select
                                                    local.set 30
                                                  end
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      local.get 31
                                                      i64.const 4294967296
                                                      i64.ge_u
                                                      br_if 0 (;@25;)
                                                      local.get 31
                                                      local.set 32
                                                      local.get 15
                                                      local.set 19
                                                      br 1 (;@24;)
                                                    end
                                                    local.get 15
                                                    local.set 19
                                                    loop ;; label = @25
                                                      local.get 19
                                                      i32.const -1
                                                      i32.add
                                                      local.tee 19
                                                      local.get 31
                                                      local.get 31
                                                      i64.const 10
                                                      i64.div_u
                                                      local.tee 32
                                                      i64.const 10
                                                      i64.mul
                                                      i64.sub
                                                      i32.wrap_i64
                                                      i32.const 48
                                                      i32.or
                                                      i32.store8
                                                      local.get 31
                                                      i64.const 42949672959
                                                      i64.gt_u
                                                      local.set 18
                                                      local.get 32
                                                      local.set 31
                                                      local.get 18
                                                      br_if 0 (;@25;)
                                                    end
                                                  end
                                                  local.get 32
                                                  i64.eqz
                                                  br_if 0 (;@23;)
                                                  local.get 32
                                                  i32.wrap_i64
                                                  local.set 18
                                                  loop ;; label = @24
                                                    local.get 19
                                                    i32.const -1
                                                    i32.add
                                                    local.tee 19
                                                    local.get 18
                                                    local.get 18
                                                    i32.const 10
                                                    i32.div_u
                                                    local.tee 21
                                                    i32.const 10
                                                    i32.mul
                                                    i32.sub
                                                    i32.const 48
                                                    i32.or
                                                    i32.store8
                                                    local.get 18
                                                    i32.const 9
                                                    i32.gt_u
                                                    local.set 24
                                                    local.get 21
                                                    local.set 18
                                                    local.get 24
                                                    br_if 0 (;@24;)
                                                  end
                                                end
                                                local.get 26
                                                local.get 23
                                                i32.const 0
                                                i32.lt_s
                                                i32.and
                                                br_if 18 (;@4;)
                                                local.get 28
                                                i32.const -65537
                                                i32.and
                                                local.get 28
                                                local.get 26
                                                select
                                                local.set 27
                                                block ;; label = @23
                                                  local.get 5
                                                  i64.load offset=40
                                                  local.tee 31
                                                  i64.const 0
                                                  i64.ne
                                                  br_if 0 (;@23;)
                                                  i32.const 0
                                                  local.set 24
                                                  local.get 23
                                                  br_if 0 (;@23;)
                                                  local.get 15
                                                  local.set 19
                                                  local.get 15
                                                  local.set 18
                                                  br 17 (;@6;)
                                                end
                                                local.get 23
                                                local.get 15
                                                local.get 19
                                                i32.sub
                                                local.get 31
                                                i64.eqz
                                                i32.add
                                                local.tee 18
                                                local.get 23
                                                local.get 18
                                                i32.gt_s
                                                select
                                                local.set 24
                                                local.get 15
                                                local.set 18
                                                br 16 (;@6;)
                                              end
                                              local.get 5
                                              i32.load8_u offset=40
                                              local.set 18
                                              br 14 (;@7;)
                                            end
                                            i32.const 0
                                            i32.load offset=4100
                                            call $strerror
                                            local.set 19
                                            br 1 (;@19;)
                                          end
                                          local.get 5
                                          i32.load offset=40
                                          local.tee 18
                                          i32.const 1163
                                          local.get 18
                                          select
                                          local.set 19
                                        end
                                        local.get 19
                                        local.get 19
                                        local.get 23
                                        i32.const 2147483647
                                        local.get 23
                                        i32.const 2147483647
                                        i32.lt_u
                                        select
                                        call $strnlen
                                        local.tee 24
                                        i32.add
                                        local.set 18
                                        i32.const 0
                                        local.set 22
                                        i32.const 1024
                                        local.set 30
                                        local.get 23
                                        i32.const -1
                                        i32.gt_s
                                        br_if 12 (;@6;)
                                        local.get 18
                                        i32.load8_u
                                        i32.eqz
                                        br_if 12 (;@6;)
                                        br 14 (;@4;)
                                      end
                                      local.get 5
                                      i64.load offset=40
                                      local.tee 31
                                      i64.eqz
                                      i32.eqz
                                      br_if 1 (;@16;)
                                      i32.const 0
                                      local.set 18
                                      br 10 (;@7;)
                                    end
                                    block ;; label = @17
                                      local.get 23
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 5
                                      i32.load offset=40
                                      local.set 21
                                      br 2 (;@15;)
                                    end
                                    i32.const 0
                                    local.set 18
                                    local.get 0
                                    i32.const 32
                                    local.get 25
                                    i32.const 0
                                    local.get 28
                                    call $pad
                                    br 2 (;@14;)
                                  end
                                  local.get 5
                                  i32.const 0
                                  i32.store offset=12
                                  local.get 5
                                  local.get 31
                                  i64.store32 offset=8
                                  local.get 5
                                  local.get 5
                                  i32.const 8
                                  i32.add
                                  i32.store offset=40
                                  local.get 5
                                  i32.const 8
                                  i32.add
                                  local.set 21
                                  i32.const -1
                                  local.set 23
                                end
                                i32.const 0
                                local.set 18
                                local.get 21
                                local.set 19
                                block ;; label = @15
                                  loop ;; label = @16
                                    local.get 19
                                    i32.load
                                    local.tee 20
                                    i32.eqz
                                    br_if 1 (;@15;)
                                    local.get 5
                                    i32.const 4
                                    i32.add
                                    local.get 20
                                    call $wctomb
                                    local.tee 20
                                    i32.const 0
                                    i32.lt_s
                                    br_if 14 (;@2;)
                                    local.get 20
                                    local.get 23
                                    local.get 18
                                    i32.sub
                                    i32.gt_u
                                    br_if 1 (;@15;)
                                    local.get 19
                                    i32.const 4
                                    i32.add
                                    local.set 19
                                    local.get 20
                                    local.get 18
                                    i32.add
                                    local.tee 18
                                    local.get 23
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                  end
                                end
                                local.get 18
                                i32.const 0
                                i32.lt_s
                                br_if 10 (;@4;)
                                local.get 0
                                i32.const 32
                                local.get 25
                                local.get 18
                                local.get 28
                                call $pad
                                block ;; label = @15
                                  local.get 18
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 18
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                local.set 19
                                loop ;; label = @15
                                  local.get 21
                                  i32.load
                                  local.tee 20
                                  i32.eqz
                                  br_if 1 (;@14;)
                                  local.get 5
                                  i32.const 4
                                  i32.add
                                  local.get 20
                                  call $wctomb
                                  local.tee 20
                                  local.get 19
                                  i32.add
                                  local.tee 19
                                  local.get 18
                                  i32.gt_u
                                  br_if 1 (;@14;)
                                  block ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 4
                                    i32.add
                                    local.get 20
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 21
                                  i32.const 4
                                  i32.add
                                  local.set 21
                                  local.get 19
                                  local.get 18
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.const 32
                              local.get 25
                              local.get 18
                              local.get 28
                              i32.const 8192
                              i32.xor
                              call $pad
                              local.get 25
                              local.get 18
                              local.get 25
                              local.get 18
                              i32.gt_s
                              select
                              local.set 18
                              br 8 (;@5;)
                            end
                            local.get 26
                            local.get 23
                            i32.const 0
                            i32.lt_s
                            local.tee 18
                            i32.and
                            br_if 8 (;@4;)
                            local.get 5
                            f64.load offset=40
                            local.set 33
                            local.get 5
                            i32.const 0
                            i32.store offset=92
                            block ;; label = @13
                              block ;; label = @14
                                local.get 33
                                i64.reinterpret_f64
                                i64.const -1
                                i64.gt_s
                                br_if 0 (;@14;)
                                local.get 33
                                f64.neg
                                local.set 33
                                i32.const 1
                                local.set 34
                                i32.const 0
                                local.set 35
                                i32.const 1034
                                local.set 36
                                br 1 (;@13;)
                              end
                              block ;; label = @14
                                local.get 28
                                i32.const 2048
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 1
                                local.set 34
                                i32.const 0
                                local.set 35
                                i32.const 1037
                                local.set 36
                                br 1 (;@13;)
                              end
                              i32.const 1040
                              i32.const 1035
                              local.get 28
                              i32.const 1
                              i32.and
                              local.tee 34
                              select
                              local.set 36
                              local.get 34
                              i32.eqz
                              local.set 35
                            end
                            block ;; label = @13
                              local.get 33
                              f64.const inf (;=inf;)
                              f64.lt
                              br_if 0 (;@13;)
                              local.get 34
                              i32.const 3
                              i32.add
                              local.set 19
                              block ;; label = @14
                                local.get 28
                                i32.const 8192
                                i32.and
                                br_if 0 (;@14;)
                                local.get 25
                                local.get 19
                                i32.le_u
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  local.get 25
                                  local.get 19
                                  i32.sub
                                  local.tee 18
                                  i32.const 256
                                  local.get 18
                                  i32.const 256
                                  i32.lt_u
                                  local.tee 20
                                  select
                                  local.tee 21
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 608
                                  i32.add
                                  i32.const 32
                                  local.get 21
                                  memory.fill
                                end
                                block ;; label = @15
                                  local.get 20
                                  br_if 0 (;@15;)
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 0
                                      i32.load8_u
                                      i32.const 32
                                      i32.and
                                      br_if 0 (;@17;)
                                      local.get 5
                                      i32.const 608
                                      i32.add
                                      i32.const 256
                                      local.get 0
                                      call $__fwritex
                                      drop
                                    end
                                    local.get 18
                                    i32.const -256
                                    i32.add
                                    local.tee 18
                                    i32.const 255
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                  end
                                  local.get 0
                                  i32.load
                                  local.set 22
                                end
                                local.get 22
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 608
                                i32.add
                                local.get 18
                                local.get 0
                                call $__fwritex
                                drop
                                local.get 0
                                i32.load
                                local.set 22
                              end
                              block ;; label = @14
                                local.get 22
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                local.get 36
                                local.get 34
                                local.get 0
                                call $__fwritex
                                drop
                                local.get 0
                                i32.load
                                local.set 22
                              end
                              block ;; label = @14
                                local.get 22
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                i32.const 1145
                                i32.const 1153
                                local.get 29
                                i32.const 32
                                i32.and
                                local.tee 18
                                select
                                i32.const 1149
                                i32.const 1157
                                local.get 18
                                select
                                local.get 33
                                local.get 33
                                f64.ne
                                select
                                i32.const 3
                                local.get 0
                                call $__fwritex
                                drop
                              end
                              block ;; label = @14
                                local.get 28
                                i32.const 73728
                                i32.and
                                i32.const 8192
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 25
                                local.get 19
                                i32.le_u
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  local.get 25
                                  local.get 19
                                  i32.sub
                                  local.tee 18
                                  i32.const 256
                                  local.get 18
                                  i32.const 256
                                  i32.lt_u
                                  local.tee 20
                                  select
                                  local.tee 21
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 608
                                  i32.add
                                  i32.const 32
                                  local.get 21
                                  memory.fill
                                end
                                block ;; label = @15
                                  local.get 20
                                  br_if 0 (;@15;)
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 0
                                      i32.load8_u
                                      i32.const 32
                                      i32.and
                                      br_if 0 (;@17;)
                                      local.get 5
                                      i32.const 608
                                      i32.add
                                      i32.const 256
                                      local.get 0
                                      call $__fwritex
                                      drop
                                    end
                                    local.get 18
                                    i32.const -256
                                    i32.add
                                    local.tee 18
                                    i32.const 255
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                  end
                                end
                                local.get 0
                                i32.load8_u
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 608
                                i32.add
                                local.get 18
                                local.get 0
                                call $__fwritex
                                drop
                              end
                              local.get 25
                              local.get 19
                              local.get 25
                              local.get 19
                              i32.gt_u
                              select
                              local.set 18
                              br 8 (;@5;)
                            end
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 33
                                  local.get 5
                                  i32.const 92
                                  i32.add
                                  call $frexp
                                  local.tee 33
                                  local.get 33
                                  f64.add
                                  local.tee 33
                                  f64.const 0x0p+0 (;=0;)
                                  f64.eq
                                  br_if 0 (;@15;)
                                  local.get 5
                                  local.get 5
                                  i32.load offset=92
                                  local.tee 19
                                  i32.const -1
                                  i32.add
                                  i32.store offset=92
                                  local.get 29
                                  i32.const 32
                                  i32.or
                                  local.tee 37
                                  i32.const 97
                                  i32.ne
                                  br_if 1 (;@14;)
                                  br 7 (;@8;)
                                end
                                local.get 29
                                i32.const 32
                                i32.or
                                local.tee 37
                                i32.const 97
                                i32.eq
                                br_if 6 (;@8;)
                                i32.const 6
                                local.get 23
                                local.get 18
                                select
                                local.set 22
                                local.get 5
                                i32.load offset=92
                                local.set 21
                                br 1 (;@13;)
                              end
                              local.get 5
                              local.get 19
                              i32.const -29
                              i32.add
                              local.tee 21
                              i32.store offset=92
                              i32.const 6
                              local.get 23
                              local.get 18
                              select
                              local.set 22
                              local.get 33
                              f64.const 0x1p+28 (;=268435456;)
                              f64.mul
                              local.set 33
                            end
                            local.get 5
                            i32.const 96
                            i32.add
                            i32.const 0
                            i32.const 288
                            local.get 21
                            i32.const 0
                            i32.lt_s
                            local.tee 38
                            select
                            i32.add
                            local.tee 30
                            local.set 19
                            loop ;; label = @13
                              local.get 19
                              local.get 33
                              i32.trunc_sat_f64_u
                              local.tee 18
                              i32.store
                              local.get 19
                              i32.const 4
                              i32.add
                              local.set 19
                              local.get 33
                              local.get 18
                              f64.convert_i32_u
                              f64.sub
                              f64.const 0x1.dcd65p+29 (;=1000000000;)
                              f64.mul
                              local.tee 33
                              f64.const 0x0p+0 (;=0;)
                              f64.ne
                              br_if 0 (;@13;)
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 21
                                i32.const 1
                                i32.ge_s
                                br_if 0 (;@14;)
                                local.get 19
                                local.set 18
                                local.get 30
                                local.set 20
                                br 1 (;@13;)
                              end
                              local.get 30
                              local.set 20
                              loop ;; label = @14
                                local.get 21
                                i32.const 29
                                local.get 21
                                i32.const 29
                                i32.lt_u
                                select
                                local.set 21
                                block ;; label = @15
                                  local.get 19
                                  i32.const -4
                                  i32.add
                                  local.tee 18
                                  local.get 20
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 21
                                  i64.extend_i32_u
                                  local.set 39
                                  i64.const 0
                                  local.set 31
                                  loop ;; label = @16
                                    local.get 18
                                    local.get 18
                                    i64.load32_u
                                    local.get 39
                                    i64.shl
                                    local.get 31
                                    i64.add
                                    local.tee 32
                                    local.get 32
                                    i64.const 1000000000
                                    i64.div_u
                                    local.tee 31
                                    i64.const 1000000000
                                    i64.mul
                                    i64.sub
                                    i64.store32
                                    local.get 18
                                    i32.const -4
                                    i32.add
                                    local.tee 18
                                    local.get 20
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                  end
                                  local.get 32
                                  i64.const 1000000000
                                  i64.lt_u
                                  br_if 0 (;@15;)
                                  local.get 20
                                  i32.const -4
                                  i32.add
                                  local.tee 20
                                  local.get 31
                                  i64.store32
                                end
                                block ;; label = @15
                                  loop ;; label = @16
                                    local.get 19
                                    local.tee 18
                                    local.get 20
                                    i32.le_u
                                    br_if 1 (;@15;)
                                    local.get 18
                                    i32.const -4
                                    i32.add
                                    local.tee 19
                                    i32.load
                                    i32.eqz
                                    br_if 0 (;@16;)
                                  end
                                end
                                local.get 5
                                local.get 5
                                i32.load offset=92
                                local.get 21
                                i32.sub
                                local.tee 21
                                i32.store offset=92
                                local.get 18
                                local.set 19
                                local.get 21
                                i32.const 0
                                i32.gt_s
                                br_if 0 (;@14;)
                              end
                            end
                            block ;; label = @13
                              local.get 21
                              i32.const -1
                              i32.gt_s
                              br_if 0 (;@13;)
                              local.get 22
                              i32.const 25
                              i32.add
                              i32.const 9
                              i32.div_u
                              i32.const 1
                              i32.add
                              local.set 40
                              local.get 37
                              i32.const 102
                              i32.eq
                              local.set 41
                              loop ;; label = @14
                                i32.const 0
                                local.get 21
                                i32.sub
                                local.tee 19
                                i32.const 9
                                local.get 19
                                i32.const 9
                                i32.lt_u
                                select
                                local.set 23
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 20
                                    local.get 18
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    i32.const 4
                                    local.get 20
                                    i32.load
                                    select
                                    local.set 19
                                    br 1 (;@15;)
                                  end
                                  i32.const 1000000000
                                  local.get 23
                                  i32.shr_u
                                  local.set 27
                                  i32.const -1
                                  local.get 23
                                  i32.shl
                                  i32.const -1
                                  i32.xor
                                  local.set 26
                                  i32.const 0
                                  local.set 21
                                  local.get 20
                                  local.set 19
                                  loop ;; label = @16
                                    local.get 19
                                    local.get 19
                                    i32.load
                                    local.tee 24
                                    local.get 23
                                    i32.shr_u
                                    local.get 21
                                    i32.add
                                    i32.store
                                    local.get 24
                                    local.get 26
                                    i32.and
                                    local.get 27
                                    i32.mul
                                    local.set 21
                                    local.get 19
                                    i32.const 4
                                    i32.add
                                    local.tee 19
                                    local.get 18
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                  end
                                  i32.const 0
                                  i32.const 4
                                  local.get 20
                                  i32.load
                                  select
                                  local.set 19
                                  local.get 21
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 18
                                  local.get 21
                                  i32.store
                                  local.get 18
                                  i32.const 4
                                  i32.add
                                  local.set 18
                                end
                                local.get 5
                                local.get 5
                                i32.load offset=92
                                local.get 23
                                i32.add
                                local.tee 21
                                i32.store offset=92
                                local.get 30
                                local.get 20
                                local.get 19
                                i32.add
                                local.tee 20
                                local.get 41
                                select
                                local.tee 19
                                local.get 40
                                i32.const 2
                                i32.shl
                                i32.add
                                local.get 18
                                local.get 18
                                local.get 19
                                i32.sub
                                i32.const 2
                                i32.shr_s
                                local.get 40
                                i32.gt_s
                                select
                                local.set 18
                                local.get 21
                                i32.const 0
                                i32.lt_s
                                br_if 0 (;@14;)
                              end
                            end
                            i32.const 0
                            local.set 24
                            block ;; label = @13
                              local.get 20
                              local.get 18
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 30
                              local.get 20
                              i32.sub
                              i32.const 2
                              i32.shr_s
                              i32.const 9
                              i32.mul
                              local.set 24
                              local.get 20
                              i32.load
                              local.tee 21
                              i32.const 10
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 10
                              local.set 19
                              loop ;; label = @14
                                local.get 24
                                i32.const 1
                                i32.add
                                local.set 24
                                local.get 21
                                local.get 19
                                i32.const 10
                                i32.mul
                                local.tee 19
                                i32.ge_u
                                br_if 0 (;@14;)
                              end
                            end
                            block ;; label = @13
                              local.get 22
                              i32.const 0
                              local.get 24
                              local.get 37
                              i32.const 102
                              i32.eq
                              select
                              i32.sub
                              local.get 22
                              i32.const 0
                              i32.ne
                              local.get 37
                              i32.const 103
                              i32.eq
                              local.tee 26
                              i32.and
                              i32.sub
                              local.tee 19
                              local.get 18
                              local.get 30
                              i32.sub
                              i32.const 2
                              i32.shr_s
                              i32.const 9
                              i32.mul
                              i32.const -9
                              i32.add
                              i32.ge_s
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 96
                              i32.add
                              i32.const -4092
                              i32.const -3804
                              local.get 38
                              select
                              local.tee 37
                              i32.add
                              local.get 19
                              i32.const 9216
                              i32.add
                              local.tee 21
                              i32.const 9
                              i32.div_s
                              local.tee 23
                              i32.const 2
                              i32.shl
                              local.tee 38
                              i32.add
                              local.set 27
                              i32.const 10
                              local.set 19
                              block ;; label = @14
                                local.get 21
                                local.get 23
                                i32.const 9
                                i32.mul
                                i32.sub
                                local.tee 23
                                i32.const 7
                                i32.gt_s
                                br_if 0 (;@14;)
                                i32.const 8
                                local.get 23
                                i32.sub
                                local.tee 40
                                i32.const 7
                                i32.and
                                local.set 21
                                i32.const 10
                                local.set 19
                                block ;; label = @15
                                  local.get 23
                                  i32.const -1
                                  i32.add
                                  i32.const 7
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 40
                                  i32.const -8
                                  i32.and
                                  local.set 23
                                  i32.const 10
                                  local.set 19
                                  loop ;; label = @16
                                    local.get 19
                                    i32.const 100000000
                                    i32.mul
                                    local.set 19
                                    local.get 23
                                    i32.const -8
                                    i32.add
                                    local.tee 23
                                    br_if 0 (;@16;)
                                  end
                                end
                                local.get 21
                                i32.eqz
                                br_if 0 (;@14;)
                                loop ;; label = @15
                                  local.get 19
                                  i32.const 10
                                  i32.mul
                                  local.set 19
                                  local.get 21
                                  i32.const -1
                                  i32.add
                                  local.tee 21
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 27
                              i32.const 4
                              i32.add
                              local.set 40
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 27
                                  i32.load
                                  local.tee 21
                                  local.get 21
                                  local.get 19
                                  i32.div_u
                                  local.tee 41
                                  local.get 19
                                  i32.mul
                                  i32.sub
                                  local.tee 23
                                  br_if 0 (;@15;)
                                  local.get 40
                                  local.get 18
                                  i32.eq
                                  br_if 1 (;@14;)
                                end
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 41
                                    i32.const 1
                                    i32.and
                                    br_if 0 (;@16;)
                                    f64.const 0x1p+53 (;=9007199254740992;)
                                    local.set 33
                                    local.get 19
                                    i32.const 1000000000
                                    i32.ne
                                    br_if 1 (;@15;)
                                    local.get 27
                                    local.get 20
                                    i32.le_u
                                    br_if 1 (;@15;)
                                    local.get 27
                                    i32.const -4
                                    i32.add
                                    i32.load8_u
                                    i32.const 1
                                    i32.and
                                    i32.eqz
                                    br_if 1 (;@15;)
                                  end
                                  f64.const 0x1.0000000000001p+53 (;=9007199254740994;)
                                  local.set 33
                                end
                                f64.const 0x1p-1 (;=0.5;)
                                f64.const 0x1p+0 (;=1;)
                                f64.const 0x1.8p+0 (;=1.5;)
                                local.get 40
                                local.get 18
                                i32.eq
                                select
                                f64.const 0x1.8p+0 (;=1.5;)
                                local.get 23
                                local.get 19
                                i32.const 1
                                i32.shr_u
                                local.tee 40
                                i32.eq
                                select
                                local.get 23
                                local.get 40
                                i32.lt_u
                                select
                                local.set 42
                                block ;; label = @15
                                  local.get 35
                                  br_if 0 (;@15;)
                                  local.get 36
                                  i32.load8_u
                                  i32.const 45
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 42
                                  f64.neg
                                  local.set 42
                                  local.get 33
                                  f64.neg
                                  local.set 33
                                end
                                local.get 27
                                local.get 21
                                local.get 23
                                i32.sub
                                local.tee 21
                                i32.store
                                local.get 33
                                local.get 42
                                f64.add
                                local.get 33
                                f64.eq
                                br_if 0 (;@14;)
                                local.get 27
                                local.get 21
                                local.get 19
                                i32.add
                                local.tee 19
                                i32.store
                                block ;; label = @15
                                  local.get 19
                                  i32.const 1000000000
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 7
                                  local.get 37
                                  local.get 38
                                  i32.add
                                  i32.add
                                  local.set 19
                                  loop ;; label = @16
                                    local.get 19
                                    i32.const 4
                                    i32.add
                                    i32.const 0
                                    i32.store
                                    block ;; label = @17
                                      local.get 19
                                      local.get 20
                                      i32.ge_u
                                      br_if 0 (;@17;)
                                      local.get 20
                                      i32.const -4
                                      i32.add
                                      local.tee 20
                                      i32.const 0
                                      i32.store
                                    end
                                    local.get 19
                                    local.get 19
                                    i32.load
                                    i32.const 1
                                    i32.add
                                    local.tee 21
                                    i32.store
                                    local.get 19
                                    i32.const -4
                                    i32.add
                                    local.set 19
                                    local.get 21
                                    i32.const 999999999
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                  end
                                  local.get 19
                                  i32.const 4
                                  i32.add
                                  local.set 27
                                end
                                local.get 30
                                local.get 20
                                i32.sub
                                i32.const 2
                                i32.shr_s
                                i32.const 9
                                i32.mul
                                local.set 24
                                local.get 20
                                i32.load
                                local.tee 21
                                i32.const 10
                                i32.lt_u
                                br_if 0 (;@14;)
                                i32.const 10
                                local.set 19
                                loop ;; label = @15
                                  local.get 24
                                  i32.const 1
                                  i32.add
                                  local.set 24
                                  local.get 21
                                  local.get 19
                                  i32.const 10
                                  i32.mul
                                  local.tee 19
                                  i32.ge_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 27
                              i32.const 4
                              i32.add
                              local.tee 19
                              local.get 18
                              local.get 18
                              local.get 19
                              i32.gt_u
                              select
                              local.set 18
                            end
                            local.get 18
                            local.get 30
                            i32.sub
                            local.set 19
                            block ;; label = @13
                              loop ;; label = @14
                                local.get 19
                                local.set 21
                                local.get 18
                                local.tee 27
                                local.get 20
                                i32.le_u
                                local.tee 23
                                br_if 1 (;@13;)
                                local.get 21
                                i32.const -4
                                i32.add
                                local.set 19
                                local.get 27
                                i32.const -4
                                i32.add
                                local.tee 18
                                i32.load
                                i32.eqz
                                br_if 0 (;@14;)
                              end
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 26
                                br_if 0 (;@14;)
                                local.get 28
                                i32.const 8
                                i32.and
                                local.set 40
                                br 1 (;@13;)
                              end
                              local.get 24
                              i32.const -1
                              i32.xor
                              i32.const -1
                              local.get 22
                              i32.const 1
                              local.get 22
                              select
                              local.tee 18
                              local.get 24
                              i32.gt_s
                              local.get 24
                              i32.const -5
                              i32.gt_s
                              i32.and
                              local.tee 19
                              select
                              local.get 18
                              i32.add
                              local.set 22
                              i32.const -1
                              i32.const -2
                              local.get 19
                              select
                              local.get 29
                              i32.add
                              local.set 29
                              local.get 28
                              i32.const 8
                              i32.and
                              local.tee 40
                              br_if 0 (;@13;)
                              i32.const -9
                              local.set 18
                              block ;; label = @14
                                local.get 23
                                br_if 0 (;@14;)
                                local.get 27
                                i32.const -4
                                i32.add
                                i32.load
                                local.tee 23
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 18
                                local.get 23
                                i32.const 10
                                i32.rem_u
                                br_if 0 (;@14;)
                                i32.const 10
                                local.set 19
                                i32.const 0
                                local.set 18
                                loop ;; label = @15
                                  local.get 18
                                  i32.const -1
                                  i32.add
                                  local.set 18
                                  local.get 23
                                  local.get 19
                                  i32.const 10
                                  i32.mul
                                  local.tee 19
                                  i32.rem_u
                                  i32.eqz
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 21
                              i32.const 2
                              i32.shr_s
                              i32.const 9
                              i32.mul
                              local.set 19
                              block ;; label = @14
                                local.get 29
                                i32.const -33
                                i32.and
                                i32.const 70
                                i32.ne
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 40
                                local.get 22
                                local.get 19
                                local.get 18
                                i32.add
                                i32.const -9
                                i32.add
                                local.tee 18
                                i32.const 0
                                local.get 18
                                i32.const 0
                                i32.gt_s
                                select
                                local.tee 18
                                local.get 22
                                local.get 18
                                i32.lt_s
                                select
                                local.set 22
                                br 1 (;@13;)
                              end
                              i32.const 0
                              local.set 40
                              local.get 22
                              local.get 24
                              local.get 19
                              i32.add
                              local.get 18
                              i32.add
                              i32.const -9
                              i32.add
                              local.tee 18
                              i32.const 0
                              local.get 18
                              i32.const 0
                              i32.gt_s
                              select
                              local.tee 18
                              local.get 22
                              local.get 18
                              i32.lt_s
                              select
                              local.set 22
                            end
                            local.get 22
                            i32.const 2147483645
                            i32.const 2147483646
                            local.get 22
                            local.get 40
                            i32.or
                            local.tee 37
                            select
                            i32.gt_s
                            br_if 8 (;@4;)
                            local.get 22
                            local.get 37
                            i32.const 0
                            i32.ne
                            i32.add
                            i32.const 1
                            i32.add
                            local.set 41
                            block ;; label = @13
                              block ;; label = @14
                                local.get 29
                                i32.const -33
                                i32.and
                                i32.const 70
                                i32.ne
                                local.tee 38
                                br_if 0 (;@14;)
                                local.get 24
                                local.get 41
                                i32.const 2147483647
                                i32.xor
                                i32.gt_s
                                br_if 10 (;@4;)
                                local.get 24
                                i32.const 0
                                local.get 24
                                i32.const 0
                                i32.gt_s
                                select
                                local.set 18
                                br 1 (;@13;)
                              end
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 24
                                  br_if 0 (;@15;)
                                  local.get 6
                                  local.set 21
                                  local.get 6
                                  local.set 19
                                  br 1 (;@14;)
                                end
                                local.get 24
                                local.get 24
                                i32.const 31
                                i32.shr_s
                                local.tee 18
                                i32.xor
                                local.get 18
                                i32.sub
                                local.set 18
                                local.get 6
                                local.set 21
                                local.get 6
                                local.set 19
                                loop ;; label = @15
                                  local.get 19
                                  i32.const -1
                                  i32.add
                                  local.tee 19
                                  local.get 18
                                  local.get 18
                                  i32.const 10
                                  i32.div_u
                                  local.tee 23
                                  i32.const 10
                                  i32.mul
                                  i32.sub
                                  i32.const 48
                                  i32.or
                                  i32.store8
                                  local.get 21
                                  i32.const -1
                                  i32.add
                                  local.set 21
                                  local.get 18
                                  i32.const 9
                                  i32.gt_u
                                  local.set 26
                                  local.get 23
                                  local.set 18
                                  local.get 26
                                  br_if 0 (;@15;)
                                end
                              end
                              block ;; label = @14
                                local.get 6
                                local.get 21
                                i32.sub
                                i32.const 1
                                i32.gt_s
                                br_if 0 (;@14;)
                                local.get 19
                                local.get 14
                                local.get 21
                                i32.sub
                                i32.add
                                local.set 19
                                local.get 21
                                local.get 5
                                i32.const 52
                                i32.add
                                i32.sub
                                i32.const -10
                                i32.add
                                local.tee 18
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 19
                                i32.const 48
                                local.get 18
                                memory.fill
                              end
                              local.get 19
                              i32.const -2
                              i32.add
                              local.tee 35
                              local.get 29
                              i32.store8
                              local.get 19
                              i32.const -1
                              i32.add
                              i32.const 45
                              i32.const 43
                              local.get 24
                              i32.const 0
                              i32.lt_s
                              select
                              i32.store8
                              local.get 6
                              local.get 35
                              i32.sub
                              local.tee 18
                              local.get 41
                              i32.const 2147483647
                              i32.xor
                              i32.gt_s
                              br_if 9 (;@4;)
                            end
                            local.get 18
                            local.get 41
                            i32.add
                            local.tee 18
                            local.get 34
                            i32.const 2147483647
                            i32.xor
                            i32.gt_s
                            br_if 8 (;@4;)
                            local.get 18
                            local.get 34
                            i32.add
                            local.set 26
                            block ;; label = @13
                              local.get 28
                              i32.const 73728
                              i32.and
                              local.tee 28
                              br_if 0 (;@13;)
                              local.get 25
                              local.get 26
                              i32.le_s
                              br_if 0 (;@13;)
                              block ;; label = @14
                                local.get 25
                                local.get 26
                                i32.sub
                                local.tee 18
                                i32.const 256
                                local.get 18
                                i32.const 256
                                i32.lt_u
                                local.tee 19
                                select
                                local.tee 21
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 608
                                i32.add
                                i32.const 32
                                local.get 21
                                memory.fill
                              end
                              block ;; label = @14
                                local.get 19
                                br_if 0 (;@14;)
                                loop ;; label = @15
                                  block ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 608
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 18
                                  i32.const -256
                                  i32.add
                                  local.tee 18
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 608
                              i32.add
                              local.get 18
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block ;; label = @13
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 36
                              local.get 34
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block ;; label = @13
                              local.get 28
                              i32.const 65536
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 25
                              local.get 26
                              i32.le_s
                              br_if 0 (;@13;)
                              block ;; label = @14
                                local.get 25
                                local.get 26
                                i32.sub
                                local.tee 18
                                i32.const 256
                                local.get 18
                                i32.const 256
                                i32.lt_u
                                local.tee 19
                                select
                                local.tee 21
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 608
                                i32.add
                                i32.const 48
                                local.get 21
                                memory.fill
                              end
                              block ;; label = @14
                                local.get 19
                                br_if 0 (;@14;)
                                loop ;; label = @15
                                  block ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 608
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 18
                                  i32.const -256
                                  i32.add
                                  local.tee 18
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 608
                              i32.add
                              local.get 18
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            local.get 38
                            br_if 2 (;@10;)
                            local.get 30
                            local.get 20
                            local.get 20
                            local.get 30
                            i32.gt_u
                            select
                            local.tee 24
                            local.set 23
                            loop ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 23
                                      i32.load
                                      local.tee 18
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      i32.const 8
                                      local.set 19
                                      loop ;; label = @18
                                        local.get 5
                                        i32.const 64
                                        i32.add
                                        local.get 19
                                        i32.add
                                        local.get 18
                                        local.get 18
                                        i32.const 10
                                        i32.div_u
                                        local.tee 20
                                        i32.const 10
                                        i32.mul
                                        i32.sub
                                        i32.const 48
                                        i32.or
                                        i32.store8
                                        local.get 19
                                        i32.const -1
                                        i32.add
                                        local.set 19
                                        local.get 18
                                        i32.const 9
                                        i32.gt_u
                                        local.set 21
                                        local.get 20
                                        local.set 18
                                        local.get 21
                                        br_if 0 (;@18;)
                                      end
                                      local.get 19
                                      i32.const 1
                                      i32.add
                                      local.tee 20
                                      local.get 5
                                      i32.const 64
                                      i32.add
                                      i32.add
                                      local.set 18
                                      block ;; label = @18
                                        local.get 23
                                        local.get 24
                                        i32.eq
                                        br_if 0 (;@18;)
                                        local.get 19
                                        i32.const 2
                                        i32.add
                                        i32.const 2
                                        i32.lt_s
                                        br_if 4 (;@14;)
                                        br 3 (;@15;)
                                      end
                                      local.get 19
                                      i32.const 8
                                      i32.ne
                                      br_if 3 (;@14;)
                                      br 1 (;@16;)
                                    end
                                    i32.const 9
                                    local.set 20
                                    local.get 23
                                    local.get 24
                                    i32.ne
                                    br_if 1 (;@15;)
                                  end
                                  local.get 5
                                  i32.const 48
                                  i32.store8 offset=72
                                  local.get 12
                                  local.set 18
                                  br 1 (;@14;)
                                end
                                local.get 20
                                local.get 5
                                i32.const 64
                                i32.add
                                i32.add
                                local.get 5
                                i32.const 64
                                i32.add
                                local.get 11
                                local.get 20
                                i32.add
                                local.tee 18
                                local.get 5
                                i32.const 64
                                i32.add
                                local.get 18
                                i32.lt_u
                                select
                                local.tee 18
                                i32.sub
                                local.tee 19
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 18
                                i32.const 48
                                local.get 19
                                memory.fill
                              end
                              block ;; label = @14
                                local.get 0
                                i32.load8_u
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                local.get 18
                                local.get 13
                                local.get 18
                                i32.sub
                                local.get 0
                                call $__fwritex
                                drop
                              end
                              local.get 23
                              i32.const 4
                              i32.add
                              local.tee 23
                              local.get 30
                              i32.le_u
                              br_if 0 (;@13;)
                            end
                            block ;; label = @13
                              local.get 37
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 1161
                              i32.const 1
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 22
                                i32.const 1
                                i32.ge_s
                                br_if 0 (;@14;)
                                local.get 22
                                local.set 18
                                br 1 (;@13;)
                              end
                              block ;; label = @14
                                local.get 23
                                local.get 27
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 22
                                local.set 18
                                br 1 (;@13;)
                              end
                              loop ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 23
                                      i32.load
                                      local.tee 18
                                      br_if 0 (;@17;)
                                      local.get 13
                                      local.set 19
                                      local.get 13
                                      local.set 20
                                      br 1 (;@16;)
                                    end
                                    local.get 13
                                    local.set 20
                                    local.get 13
                                    local.set 19
                                    loop ;; label = @17
                                      local.get 19
                                      i32.const -1
                                      i32.add
                                      local.tee 19
                                      local.get 18
                                      local.get 18
                                      i32.const 10
                                      i32.div_u
                                      local.tee 21
                                      i32.const 10
                                      i32.mul
                                      i32.sub
                                      i32.const 48
                                      i32.or
                                      i32.store8
                                      local.get 20
                                      i32.const -1
                                      i32.add
                                      local.set 20
                                      local.get 18
                                      i32.const 9
                                      i32.gt_u
                                      local.set 24
                                      local.get 21
                                      local.set 18
                                      local.get 24
                                      br_if 0 (;@17;)
                                    end
                                    local.get 19
                                    local.get 5
                                    i32.const 64
                                    i32.add
                                    i32.le_u
                                    br_if 1 (;@15;)
                                  end
                                  local.get 19
                                  local.get 5
                                  i32.const 64
                                  i32.add
                                  i32.add
                                  local.get 20
                                  i32.sub
                                  local.set 19
                                  local.get 20
                                  local.get 5
                                  i32.const 64
                                  i32.add
                                  i32.sub
                                  local.tee 18
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 19
                                  i32.const 48
                                  local.get 18
                                  memory.fill
                                end
                                block ;; label = @15
                                  local.get 0
                                  i32.load8_u
                                  i32.const 32
                                  i32.and
                                  br_if 0 (;@15;)
                                  local.get 19
                                  local.get 22
                                  i32.const 9
                                  local.get 22
                                  i32.const 9
                                  i32.lt_u
                                  select
                                  local.get 0
                                  call $__fwritex
                                  drop
                                end
                                local.get 22
                                i32.const -9
                                i32.add
                                local.set 18
                                local.get 23
                                i32.const 4
                                i32.add
                                local.tee 23
                                local.get 27
                                i32.ge_u
                                br_if 1 (;@13;)
                                local.get 22
                                i32.const 9
                                i32.gt_s
                                local.set 19
                                local.get 18
                                local.set 22
                                local.get 19
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.const 48
                            local.get 18
                            i32.const 9
                            i32.add
                            i32.const 9
                            i32.const 0
                            call $pad
                            br 3 (;@9;)
                          end
                          i32.const 0
                          i32.const 28
                          i32.store offset=4100
                          br 9 (;@2;)
                        end
                        i32.const 0
                        local.set 22
                        i32.const 1024
                        local.set 30
                        local.get 15
                        local.set 18
                        local.get 28
                        local.set 27
                        local.get 23
                        local.set 24
                        br 4 (;@6;)
                      end
                      block ;; label = @10
                        local.get 22
                        i32.const 0
                        i32.lt_s
                        br_if 0 (;@10;)
                        local.get 27
                        local.get 20
                        i32.const 4
                        i32.add
                        local.get 27
                        local.get 20
                        i32.gt_u
                        select
                        local.set 27
                        local.get 20
                        local.set 23
                        loop ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 23
                              i32.load
                              local.tee 18
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 13
                              local.set 19
                              loop ;; label = @14
                                local.get 19
                                i32.const -1
                                i32.add
                                local.tee 19
                                local.get 18
                                local.get 18
                                i32.const 10
                                i32.div_u
                                local.tee 21
                                i32.const 10
                                i32.mul
                                i32.sub
                                i32.const 48
                                i32.or
                                i32.store8
                                local.get 18
                                i32.const 10
                                i32.lt_u
                                local.set 24
                                local.get 21
                                local.set 18
                                local.get 24
                                i32.eqz
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 5
                            i32.const 48
                            i32.store8 offset=72
                            local.get 12
                            local.set 19
                          end
                          block ;; label = @12
                            block ;; label = @13
                              local.get 23
                              local.get 20
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 19
                              local.get 5
                              i32.const 64
                              i32.add
                              i32.le_u
                              br_if 1 (;@12;)
                              block ;; label = @14
                                local.get 19
                                local.get 5
                                i32.const 64
                                i32.add
                                i32.sub
                                local.tee 18
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 64
                                i32.add
                                i32.const 48
                                local.get 18
                                memory.fill
                              end
                              local.get 5
                              i32.const 64
                              i32.add
                              local.set 19
                              br 1 (;@12;)
                            end
                            block ;; label = @13
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 19
                              i32.const 1
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            local.get 19
                            i32.const 1
                            i32.add
                            local.set 19
                            local.get 22
                            local.get 40
                            i32.or
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 1161
                            i32.const 1
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 13
                          local.get 19
                          i32.sub
                          local.set 18
                          block ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 19
                            local.get 18
                            local.get 22
                            local.get 18
                            local.get 22
                            i32.lt_s
                            select
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 22
                          local.get 18
                          i32.sub
                          local.set 22
                          local.get 23
                          i32.const 4
                          i32.add
                          local.tee 23
                          local.get 27
                          i32.ge_u
                          br_if 1 (;@10;)
                          local.get 22
                          i32.const -1
                          i32.gt_s
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 0
                      i32.const 48
                      local.get 22
                      i32.const 18
                      i32.add
                      i32.const 18
                      i32.const 0
                      call $pad
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 35
                      local.get 6
                      local.get 35
                      i32.sub
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    block ;; label = @9
                      local.get 28
                      i32.const 8192
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 25
                      local.get 26
                      i32.le_s
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 25
                        local.get 26
                        i32.sub
                        local.tee 18
                        i32.const 256
                        local.get 18
                        i32.const 256
                        i32.lt_u
                        local.tee 19
                        select
                        local.tee 20
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 5
                        i32.const 608
                        i32.add
                        i32.const 32
                        local.get 20
                        memory.fill
                      end
                      block ;; label = @10
                        local.get 19
                        br_if 0 (;@10;)
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 608
                            i32.add
                            i32.const 256
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 18
                          i32.const -256
                          i32.add
                          local.tee 18
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 608
                      i32.add
                      local.get 18
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 25
                    local.get 26
                    local.get 25
                    local.get 26
                    i32.gt_s
                    select
                    local.set 18
                    br 3 (;@5;)
                  end
                  local.get 36
                  local.get 29
                  i32.const 26
                  i32.shl
                  i32.const 31
                  i32.shr_s
                  i32.const 9
                  i32.and
                  i32.add
                  local.set 22
                  block ;; label = @8
                    local.get 23
                    i32.const 11
                    i32.gt_u
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 12
                        local.get 23
                        i32.sub
                        local.tee 18
                        i32.const 7
                        i32.and
                        local.tee 19
                        br_if 0 (;@10;)
                        f64.const 0x1p+4 (;=16;)
                        local.set 42
                        br 1 (;@9;)
                      end
                      local.get 23
                      i32.const -12
                      i32.add
                      local.set 18
                      f64.const 0x1p+4 (;=16;)
                      local.set 42
                      loop ;; label = @10
                        local.get 18
                        i32.const 1
                        i32.add
                        local.set 18
                        local.get 42
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        local.set 42
                        local.get 19
                        i32.const -1
                        i32.add
                        local.tee 19
                        br_if 0 (;@10;)
                      end
                      i32.const 0
                      local.get 18
                      i32.sub
                      local.set 18
                    end
                    block ;; label = @9
                      local.get 23
                      i32.const 4
                      i32.gt_u
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        local.get 42
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        f64.const 0x1p+4 (;=16;)
                        f64.mul
                        local.set 42
                        local.get 18
                        i32.const -8
                        i32.add
                        local.tee 18
                        br_if 0 (;@10;)
                      end
                    end
                    block ;; label = @9
                      local.get 22
                      i32.load8_u
                      i32.const 45
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 42
                      local.get 33
                      f64.neg
                      local.get 42
                      f64.sub
                      f64.add
                      f64.neg
                      local.set 33
                      br 1 (;@8;)
                    end
                    local.get 33
                    local.get 42
                    f64.add
                    local.get 42
                    f64.sub
                    local.set 33
                  end
                  block ;; label = @8
                    block ;; label = @9
                      local.get 5
                      i32.load offset=92
                      local.tee 24
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 24
                      local.get 24
                      i32.const 31
                      i32.shr_s
                      local.tee 18
                      i32.xor
                      local.get 18
                      i32.sub
                      local.set 18
                      local.get 6
                      local.set 19
                      loop ;; label = @10
                        local.get 19
                        i32.const -1
                        i32.add
                        local.tee 19
                        local.get 18
                        local.get 18
                        i32.const 10
                        i32.div_u
                        local.tee 20
                        i32.const 10
                        i32.mul
                        i32.sub
                        i32.const 48
                        i32.or
                        i32.store8
                        local.get 18
                        i32.const 10
                        i32.lt_u
                        local.set 21
                        local.get 20
                        local.set 18
                        local.get 21
                        i32.eqz
                        br_if 0 (;@10;)
                        br 2 (;@8;)
                      end
                    end
                    local.get 5
                    i32.const 48
                    i32.store8 offset=63
                    local.get 10
                    local.set 19
                  end
                  local.get 34
                  i32.const 2
                  i32.or
                  local.set 27
                  local.get 29
                  i32.const 32
                  i32.and
                  local.set 20
                  local.get 19
                  i32.const -2
                  i32.add
                  local.tee 26
                  local.get 29
                  i32.const 15
                  i32.add
                  i32.store8
                  local.get 19
                  i32.const -1
                  i32.add
                  i32.const 45
                  i32.const 43
                  local.get 24
                  i32.const 0
                  i32.lt_s
                  select
                  i32.store8
                  local.get 28
                  i32.const 8
                  i32.and
                  i32.eqz
                  local.get 23
                  i32.const 1
                  i32.lt_s
                  i32.and
                  local.set 21
                  local.get 5
                  i32.const 64
                  i32.add
                  local.set 19
                  loop ;; label = @8
                    local.get 19
                    local.tee 18
                    local.get 33
                    i32.trunc_sat_f64_s
                    local.tee 19
                    i32.const 3552
                    i32.add
                    i32.load8_u
                    local.get 20
                    i32.or
                    i32.store8
                    local.get 33
                    local.get 19
                    f64.convert_i32_s
                    f64.sub
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    local.set 33
                    block ;; label = @9
                      local.get 18
                      i32.const 1
                      i32.add
                      local.tee 19
                      local.get 5
                      i32.const 64
                      i32.add
                      i32.sub
                      i32.const 1
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 21
                      local.get 33
                      f64.const 0x0p+0 (;=0;)
                      f64.eq
                      i32.and
                      br_if 0 (;@9;)
                      local.get 18
                      i32.const 46
                      i32.store8 offset=1
                      local.get 18
                      i32.const 2
                      i32.add
                      local.set 19
                    end
                    local.get 33
                    f64.const 0x0p+0 (;=0;)
                    f64.ne
                    br_if 0 (;@8;)
                  end
                  local.get 23
                  i32.const 2147483645
                  local.get 6
                  local.get 26
                  i32.sub
                  local.tee 30
                  local.get 27
                  i32.add
                  local.tee 18
                  i32.sub
                  i32.gt_s
                  br_if 3 (;@4;)
                  local.get 23
                  i32.const 2
                  i32.add
                  local.get 19
                  local.get 5
                  i32.const 64
                  i32.add
                  i32.sub
                  local.tee 20
                  local.get 20
                  i32.const -2
                  i32.add
                  local.get 23
                  i32.lt_s
                  select
                  local.get 20
                  local.get 23
                  select
                  local.tee 24
                  local.get 18
                  i32.add
                  local.set 19
                  block ;; label = @8
                    local.get 28
                    i32.const 73728
                    i32.and
                    local.tee 21
                    br_if 0 (;@8;)
                    local.get 25
                    local.get 19
                    i32.le_s
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 25
                      local.get 19
                      i32.sub
                      local.tee 18
                      i32.const 256
                      local.get 18
                      i32.const 256
                      i32.lt_u
                      local.tee 23
                      select
                      local.tee 28
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 608
                      i32.add
                      i32.const 32
                      local.get 28
                      memory.fill
                    end
                    block ;; label = @9
                      local.get 23
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        block ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 608
                          i32.add
                          i32.const 256
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 18
                        i32.const -256
                        i32.add
                        local.tee 18
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 608
                    i32.add
                    local.get 18
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 22
                    local.get 27
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 21
                    i32.const 65536
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 25
                    local.get 19
                    i32.le_s
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 25
                      local.get 19
                      i32.sub
                      local.tee 18
                      i32.const 256
                      local.get 18
                      i32.const 256
                      i32.lt_u
                      local.tee 23
                      select
                      local.tee 27
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 608
                      i32.add
                      i32.const 48
                      local.get 27
                      memory.fill
                    end
                    block ;; label = @9
                      local.get 23
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        block ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 608
                          i32.add
                          i32.const 256
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 18
                        i32.const -256
                        i32.add
                        local.tee 18
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 608
                    i32.add
                    local.get 18
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 64
                    i32.add
                    local.get 20
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 24
                    local.get 20
                    i32.sub
                    local.tee 18
                    i32.const 1
                    i32.lt_s
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 18
                      i32.const 256
                      local.get 18
                      i32.const 256
                      i32.lt_u
                      local.tee 20
                      select
                      local.tee 24
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 608
                      i32.add
                      i32.const 48
                      local.get 24
                      memory.fill
                    end
                    block ;; label = @9
                      local.get 20
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        block ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 608
                          i32.add
                          i32.const 256
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 18
                        i32.const -256
                        i32.add
                        local.tee 18
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 608
                    i32.add
                    local.get 18
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 26
                    local.get 30
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  block ;; label = @8
                    local.get 21
                    i32.const 8192
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 25
                    local.get 19
                    i32.le_s
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 25
                      local.get 19
                      i32.sub
                      local.tee 18
                      i32.const 256
                      local.get 18
                      i32.const 256
                      i32.lt_u
                      local.tee 20
                      select
                      local.tee 21
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 608
                      i32.add
                      i32.const 32
                      local.get 21
                      memory.fill
                    end
                    block ;; label = @9
                      local.get 20
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        block ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 608
                          i32.add
                          i32.const 256
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 18
                        i32.const -256
                        i32.add
                        local.tee 18
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 608
                    i32.add
                    local.get 18
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  local.get 25
                  local.get 19
                  local.get 25
                  local.get 19
                  i32.gt_s
                  select
                  local.set 18
                  br 2 (;@5;)
                end
                local.get 5
                local.get 18
                i32.store8 offset=39
                i32.const 0
                local.set 22
                i32.const 1024
                local.set 30
                i32.const 1
                local.set 24
                local.get 9
                local.set 19
                local.get 15
                local.set 18
              end
              local.get 24
              local.get 18
              local.get 19
              i32.sub
              local.tee 23
              local.get 24
              local.get 23
              i32.gt_s
              select
              local.tee 26
              local.get 22
              i32.const 2147483647
              i32.xor
              i32.gt_s
              br_if 1 (;@4;)
              local.get 25
              local.get 22
              local.get 26
              i32.add
              local.tee 21
              local.get 25
              local.get 21
              i32.gt_s
              select
              local.tee 18
              local.get 20
              i32.gt_u
              br_if 1 (;@4;)
              block ;; label = @6
                local.get 27
                i32.const 73728
                i32.and
                local.tee 27
                br_if 0 (;@6;)
                local.get 25
                local.get 21
                i32.le_s
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 18
                  local.get 21
                  i32.sub
                  local.tee 20
                  i32.const 256
                  local.get 20
                  i32.const 256
                  i32.lt_u
                  local.tee 28
                  select
                  local.tee 40
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 96
                  i32.add
                  i32.const 32
                  local.get 40
                  memory.fill
                end
                block ;; label = @7
                  local.get 28
                  br_if 0 (;@7;)
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 96
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 20
                    i32.const -256
                    i32.add
                    local.tee 20
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 96
                i32.add
                local.get 20
                local.get 0
                call $__fwritex
                drop
              end
              block ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 30
                local.get 22
                local.get 0
                call $__fwritex
                drop
              end
              block ;; label = @6
                local.get 27
                i32.const 65536
                i32.ne
                br_if 0 (;@6;)
                local.get 25
                local.get 21
                i32.le_s
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 18
                  local.get 21
                  i32.sub
                  local.tee 20
                  i32.const 256
                  local.get 20
                  i32.const 256
                  i32.lt_u
                  local.tee 22
                  select
                  local.tee 30
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 96
                  i32.add
                  i32.const 48
                  local.get 30
                  memory.fill
                end
                block ;; label = @7
                  local.get 22
                  br_if 0 (;@7;)
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 96
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 20
                    i32.const -256
                    i32.add
                    local.tee 20
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 96
                i32.add
                local.get 20
                local.get 0
                call $__fwritex
                drop
              end
              block ;; label = @6
                local.get 24
                local.get 23
                i32.le_s
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 26
                  local.get 23
                  i32.sub
                  local.tee 20
                  i32.const 256
                  local.get 20
                  i32.const 256
                  i32.lt_u
                  local.tee 24
                  select
                  local.tee 26
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 96
                  i32.add
                  i32.const 48
                  local.get 26
                  memory.fill
                end
                block ;; label = @7
                  local.get 24
                  br_if 0 (;@7;)
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 96
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 20
                    i32.const -256
                    i32.add
                    local.tee 20
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 96
                i32.add
                local.get 20
                local.get 0
                call $__fwritex
                drop
              end
              block ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 19
                local.get 23
                local.get 0
                call $__fwritex
                drop
              end
              local.get 27
              i32.const 8192
              i32.ne
              br_if 0 (;@5;)
              local.get 25
              local.get 21
              i32.le_s
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 18
                local.get 21
                i32.sub
                local.tee 19
                i32.const 256
                local.get 19
                i32.const 256
                i32.lt_u
                local.tee 20
                select
                local.tee 21
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 96
                i32.add
                i32.const 32
                local.get 21
                memory.fill
              end
              block ;; label = @6
                local.get 20
                br_if 0 (;@6;)
                loop ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 96
                    i32.add
                    i32.const 256
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  local.get 19
                  i32.const -256
                  i32.add
                  local.tee 19
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              i32.load8_u
              i32.const 32
              i32.and
              br_if 0 (;@5;)
              local.get 5
              i32.const 96
              i32.add
              local.get 19
              local.get 0
              call $__fwritex
              drop
              br 0 (;@5;)
            end
          end
        end
        i32.const 0
        i32.const 61
        i32.store offset=4100
      end
      i32.const -1
      local.set 17
    end
    local.get 5
    i32.const 864
    i32.add
    global.set $__stack_pointer
    local.get 17
  )
  (func $pop_arg (;68;) (type 13) (param i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        block ;; label = @19
                                          local.get 1
                                          i32.const -9
                                          i32.add
                                          br_table 17 (;@2;) 0 (;@19;) 1 (;@18;) 4 (;@15;) 2 (;@17;) 3 (;@16;) 5 (;@14;) 6 (;@13;) 7 (;@12;) 8 (;@11;) 9 (;@10;) 10 (;@9;) 11 (;@8;) 12 (;@7;) 13 (;@6;) 14 (;@5;) 15 (;@4;) 16 (;@3;) 18 (;@1;)
                                        end
                                        local.get 2
                                        local.get 2
                                        i32.load
                                        local.tee 1
                                        i32.const 4
                                        i32.add
                                        i32.store
                                        local.get 0
                                        local.get 1
                                        i64.load32_s
                                        i64.store
                                        return
                                      end
                                      local.get 2
                                      local.get 2
                                      i32.load
                                      local.tee 1
                                      i32.const 4
                                      i32.add
                                      i32.store
                                      local.get 0
                                      local.get 1
                                      i64.load32_u
                                      i64.store
                                      return
                                    end
                                    local.get 2
                                    local.get 2
                                    i32.load
                                    local.tee 1
                                    i32.const 4
                                    i32.add
                                    i32.store
                                    local.get 0
                                    local.get 1
                                    i64.load32_s
                                    i64.store
                                    return
                                  end
                                  local.get 2
                                  local.get 2
                                  i32.load
                                  local.tee 1
                                  i32.const 4
                                  i32.add
                                  i32.store
                                  local.get 0
                                  local.get 1
                                  i64.load32_u
                                  i64.store
                                  return
                                end
                                local.get 2
                                local.get 2
                                i32.load
                                i32.const 7
                                i32.add
                                i32.const -8
                                i32.and
                                local.tee 1
                                i32.const 8
                                i32.add
                                i32.store
                                local.get 0
                                local.get 1
                                i64.load
                                i64.store
                                return
                              end
                              local.get 2
                              local.get 2
                              i32.load
                              local.tee 1
                              i32.const 4
                              i32.add
                              i32.store
                              local.get 0
                              local.get 1
                              i64.load16_s
                              i64.store
                              return
                            end
                            local.get 2
                            local.get 2
                            i32.load
                            local.tee 1
                            i32.const 4
                            i32.add
                            i32.store
                            local.get 0
                            local.get 1
                            i64.load16_u
                            i64.store
                            return
                          end
                          local.get 2
                          local.get 2
                          i32.load
                          local.tee 1
                          i32.const 4
                          i32.add
                          i32.store
                          local.get 0
                          local.get 1
                          i64.load8_s
                          i64.store
                          return
                        end
                        local.get 2
                        local.get 2
                        i32.load
                        local.tee 1
                        i32.const 4
                        i32.add
                        i32.store
                        local.get 0
                        local.get 1
                        i64.load8_u
                        i64.store
                        return
                      end
                      local.get 2
                      local.get 2
                      i32.load
                      i32.const 7
                      i32.add
                      i32.const -8
                      i32.and
                      local.tee 1
                      i32.const 8
                      i32.add
                      i32.store
                      local.get 0
                      local.get 1
                      i64.load
                      i64.store
                      return
                    end
                    local.get 2
                    local.get 2
                    i32.load
                    local.tee 1
                    i32.const 4
                    i32.add
                    i32.store
                    local.get 0
                    local.get 1
                    i64.load32_u
                    i64.store
                    return
                  end
                  local.get 2
                  local.get 2
                  i32.load
                  i32.const 7
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const 8
                  i32.add
                  i32.store
                  local.get 0
                  local.get 1
                  i64.load
                  i64.store
                  return
                end
                local.get 2
                local.get 2
                i32.load
                i32.const 7
                i32.add
                i32.const -8
                i32.and
                local.tee 1
                i32.const 8
                i32.add
                i32.store
                local.get 0
                local.get 1
                i64.load
                i64.store
                return
              end
              local.get 2
              local.get 2
              i32.load
              local.tee 1
              i32.const 4
              i32.add
              i32.store
              local.get 0
              local.get 1
              i64.load32_s
              i64.store
              return
            end
            local.get 2
            local.get 2
            i32.load
            local.tee 1
            i32.const 4
            i32.add
            i32.store
            local.get 0
            local.get 1
            i64.load32_u
            i64.store
            return
          end
          local.get 2
          local.get 2
          i32.load
          i32.const 7
          i32.add
          i32.const -8
          i32.and
          local.tee 1
          i32.const 8
          i32.add
          i32.store
          local.get 0
          local.get 1
          f64.load
          f64.store
          return
        end
        call $long_double_not_supported
        unreachable
      end
      local.get 2
      local.get 2
      i32.load
      local.tee 1
      i32.const 4
      i32.add
      i32.store
      local.get 0
      local.get 1
      i32.load
      i32.store
    end
  )
  (func $pad (;69;) (type 14) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 256
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      local.get 2
      local.get 3
      i32.le_s
      br_if 0 (;@1;)
      local.get 4
      i32.const 73728
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        local.get 3
        i32.sub
        local.tee 3
        i32.const 256
        local.get 3
        i32.const 256
        i32.lt_u
        local.tee 2
        select
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        local.get 4
        memory.fill
      end
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        loop ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load8_u
            i32.const 32
            i32.and
            br_if 0 (;@4;)
            local.get 5
            i32.const 256
            local.get 0
            call $__fwritex
            drop
          end
          local.get 3
          i32.const -256
          i32.add
          local.tee 3
          i32.const 255
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.load8_u
      i32.const 32
      i32.and
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      local.get 0
      call $__fwritex
      drop
    end
    local.get 5
    i32.const 256
    i32.add
    global.set $__stack_pointer
  )
  (func $long_double_not_supported (;70;) (type 3)
    i32.const 1239
    i32.const 3968
    call $fputs
    drop
    call $abort
    unreachable
  )
  (func $__toread (;71;) (type 4) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    local.get 0
    i32.load offset=60
    local.tee 1
    i32.const -1
    i32.add
    local.get 1
    i32.or
    i32.store offset=60
    block ;; label = @1
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.const 0
      local.get 0
      i32.load offset=32
      call_indirect (type 1)
      drop
    end
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i64.const 0
    i64.store offset=16
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 4
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    local.get 0
    local.get 0
    i32.load offset=40
    local.get 0
    i32.load offset=44
    i32.add
    local.tee 2
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 27
    i32.shl
    i32.const 31
    i32.shr_s
  )
  (func $__uflow (;72;) (type 4) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const -1
    local.set 2
    block ;; label = @1
      local.get 0
      call $__toread
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 15
      i32.add
      i32.const 1
      local.get 0
      i32.load offset=28
      call_indirect (type 1)
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load8_u offset=15
      local.set 2
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $__shlim (;73;) (type 15) (param i32 i64)
    (local i32 i32)
    local.get 0
    local.get 1
    i64.store offset=88
    local.get 0
    local.get 0
    i32.load offset=40
    local.get 0
    i32.load offset=4
    local.tee 2
    i32.sub
    i64.extend_i32_s
    i64.store offset=96
    local.get 0
    i32.load offset=8
    local.set 3
    block ;; label = @1
      local.get 1
      i64.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 3
      local.get 2
      i32.sub
      i64.extend_i32_s
      i64.ge_s
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.wrap_i64
      i32.add
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store offset=84
  )
  (func $__shgetc (;74;) (type 4) (param i32) (result i32)
    (local i32 i32 i64 i64 i32)
    local.get 0
    i64.load offset=96
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=40
    local.tee 2
    i32.sub
    i64.extend_i32_s
    i64.add
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i64.load offset=88
          local.tee 4
          i64.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i64.ge_s
          br_if 1 (;@2;)
        end
        local.get 0
        call $__uflow
        local.tee 2
        i32.const -1
        i32.gt_s
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 1
        local.get 0
        i32.load offset=40
        local.set 2
      end
      local.get 0
      i64.const -1
      i64.store offset=88
      local.get 0
      local.get 1
      i32.store offset=84
      local.get 0
      local.get 3
      local.get 2
      local.get 1
      i32.sub
      i64.extend_i32_s
      i64.add
      i64.store offset=96
      i32.const -1
      return
    end
    local.get 3
    i64.const 1
    i64.add
    local.set 3
    local.get 0
    i32.load offset=4
    local.set 1
    local.get 0
    i32.load offset=8
    local.set 5
    block ;; label = @1
      local.get 0
      i64.load offset=88
      local.tee 4
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      local.get 4
      local.get 3
      i64.sub
      local.tee 4
      local.get 5
      local.get 1
      i32.sub
      i64.extend_i32_s
      i64.ge_s
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      i32.wrap_i64
      i32.add
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=84
    local.get 0
    local.get 3
    local.get 0
    i32.load offset=40
    local.tee 5
    local.get 1
    i32.sub
    i64.extend_i32_s
    i64.add
    i64.store offset=96
    block ;; label = @1
      local.get 1
      local.get 5
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.get 2
      i32.store8
    end
    local.get 2
  )
  (func $__intscan (;75;) (type 16) (param i32 i32 i32 i64) (result i64)
    (local i32 i32 i32 i64 i64 i64 i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.const 36
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              block ;; label = @6
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        local.get 0
                        i32.load offset=84
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.store offset=4
                        local.get 5
                        i32.load8_u
                        local.set 5
                        br 1 (;@9;)
                      end
                      local.get 0
                      call $__shgetc
                      local.set 5
                    end
                    local.get 5
                    i32.const -9
                    i32.add
                    i32.const 5
                    i32.lt_u
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 5
                      i32.const -32
                      i32.add
                      br_table 1 (;@8;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 0 (;@9;) 2 (;@7;) 0 (;@9;) 2 (;@7;)
                    end
                  end
                  i32.const -1
                  i32.const 0
                  local.get 5
                  i32.const 45
                  i32.eq
                  select
                  local.set 6
                  block ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 2 (;@6;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 6
              end
              block ;; label = @6
                block ;; label = @7
                  local.get 1
                  i32.const 0
                  i32.ne
                  local.get 1
                  i32.const 16
                  i32.ne
                  i32.and
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 48
                  i32.ne
                  br_if 0 (;@7;)
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 5
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 5
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 5
                      i32.load8_u
                      local.set 5
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 5
                  end
                  block ;; label = @8
                    local.get 5
                    i32.const -33
                    i32.and
                    i32.const 88
                    i32.ne
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        local.get 0
                        i32.load offset=84
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.store offset=4
                        local.get 5
                        i32.load8_u
                        local.set 5
                        br 1 (;@9;)
                      end
                      local.get 0
                      call $__shgetc
                      local.set 5
                    end
                    i32.const 16
                    local.set 1
                    local.get 5
                    i32.const 3569
                    i32.add
                    i32.load8_u
                    i32.const 16
                    i32.lt_u
                    br_if 4 (;@4;)
                    i64.const 0
                    local.set 3
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i64.load offset=88
                        i64.const 0
                        i64.lt_s
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        i32.const -1
                        i32.add
                        i32.store offset=4
                        local.get 2
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 5
                        i32.const -2
                        i32.add
                        i32.store offset=4
                        br 9 (;@1;)
                      end
                      local.get 2
                      br_if 8 (;@1;)
                    end
                    i64.const 0
                    local.set 3
                    local.get 0
                    i64.const 0
                    call $__shlim
                    br 7 (;@1;)
                  end
                  local.get 1
                  br_if 1 (;@6;)
                  i32.const 8
                  local.set 1
                  br 3 (;@4;)
                end
                local.get 1
                i32.const 10
                local.get 1
                select
                local.tee 1
                local.get 5
                i32.const 3569
                i32.add
                i32.load8_u
                i32.gt_u
                br_if 0 (;@6;)
                i64.const 0
                local.set 3
                block ;; label = @7
                  local.get 0
                  i64.load offset=88
                  i64.const 0
                  i64.lt_s
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 0
                  i32.load offset=4
                  i32.const -1
                  i32.add
                  i32.store offset=4
                end
                local.get 0
                i64.const 0
                call $__shlim
                i32.const 0
                i32.const 28
                i32.store offset=4100
                br 5 (;@1;)
              end
              local.get 1
              i32.const 10
              i32.ne
              br_if 1 (;@4;)
              i64.const 0
              local.set 7
              block ;; label = @6
                local.get 5
                i32.const -48
                i32.add
                local.tee 2
                i32.const 9
                i32.gt_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 5
                loop ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 1
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 1
                  end
                  local.get 5
                  i32.const 10
                  i32.mul
                  local.get 2
                  i32.add
                  local.set 5
                  block ;; label = @8
                    local.get 1
                    i32.const -48
                    i32.add
                    local.tee 2
                    i32.const 9
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 429496729
                    i32.lt_u
                    br_if 1 (;@7;)
                  end
                end
                local.get 5
                i64.extend_i32_u
                local.set 7
              end
              local.get 2
              i32.const 9
              i32.gt_u
              br_if 3 (;@2;)
              local.get 7
              i64.const 10
              i64.mul
              local.set 8
              local.get 2
              i64.extend_i32_u
              local.set 9
              loop ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                end
                local.get 8
                local.get 9
                i64.add
                local.set 7
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.const -48
                    i32.add
                    local.tee 1
                    i32.const 9
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 7
                    i64.const 1844674407370955162
                    i64.lt_u
                    br_if 1 (;@7;)
                  end
                  local.get 1
                  i32.const 9
                  i32.gt_u
                  br_if 5 (;@2;)
                  i32.const 10
                  local.set 1
                  br 4 (;@3;)
                end
                local.get 7
                i64.const 10
                i64.mul
                local.tee 8
                local.get 1
                i64.extend_i32_u
                local.tee 9
                i64.const -1
                i64.xor
                i64.le_u
                br_if 0 (;@6;)
              end
              i32.const 10
              local.set 1
              br 2 (;@3;)
            end
            i32.const 0
            i32.const 28
            i32.store offset=4100
            i64.const 0
            local.set 3
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 1
            local.get 1
            i32.const -1
            i32.add
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i64.const 0
            local.set 7
            block ;; label = @5
              local.get 1
              local.get 5
              i32.const 3569
              i32.add
              i32.load8_u
              local.tee 10
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              loop ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                end
                local.get 10
                local.get 2
                local.get 1
                i32.mul
                i32.add
                local.set 2
                block ;; label = @7
                  local.get 1
                  local.get 5
                  i32.const 3569
                  i32.add
                  i32.load8_u
                  local.tee 10
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 119304647
                  i32.lt_u
                  br_if 1 (;@6;)
                end
              end
              local.get 2
              i64.extend_i32_u
              local.set 7
            end
            local.get 1
            local.get 10
            i32.le_u
            br_if 1 (;@3;)
            local.get 1
            i64.extend_i32_u
            local.set 8
            loop ;; label = @5
              local.get 7
              local.get 8
              i64.mul
              local.tee 9
              local.get 10
              i64.extend_i32_u
              i64.const 255
              i64.and
              local.tee 11
              i64.const -1
              i64.xor
              i64.gt_u
              br_if 2 (;@3;)
              block ;; label = @6
                block ;; label = @7
                  local.get 0
                  i32.load offset=4
                  local.tee 5
                  local.get 0
                  i32.load offset=84
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.store offset=4
                  local.get 5
                  i32.load8_u
                  local.set 5
                  br 1 (;@6;)
                end
                local.get 0
                call $__shgetc
                local.set 5
              end
              local.get 9
              local.get 11
              i64.add
              local.set 7
              local.get 1
              local.get 5
              i32.const 3569
              i32.add
              i32.load8_u
              local.tee 10
              i32.le_u
              br_if 2 (;@3;)
              local.get 4
              local.get 8
              i64.const 0
              local.get 7
              i64.const 0
              call $__multi3
              local.get 4
              i64.load offset=8
              i64.const 0
              i64.ne
              br_if 2 (;@3;)
              br 0 (;@5;)
            end
          end
          local.get 1
          i32.const 23
          i32.mul
          i32.const 5
          i32.shr_u
          i32.const 7
          i32.and
          i32.load8_s offset=3825
          local.set 12
          i64.const 0
          local.set 7
          block ;; label = @4
            local.get 1
            local.get 5
            i32.const 3569
            i32.add
            i32.load8_u
            local.tee 2
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 10
            loop ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 0
                  i32.load offset=4
                  local.tee 5
                  local.get 0
                  i32.load offset=84
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.store offset=4
                  local.get 5
                  i32.load8_u
                  local.set 5
                  br 1 (;@6;)
                end
                local.get 0
                call $__shgetc
                local.set 5
              end
              local.get 2
              local.get 10
              local.get 12
              i32.shl
              local.tee 13
              i32.or
              local.set 10
              block ;; label = @6
                local.get 1
                local.get 5
                i32.const 3569
                i32.add
                i32.load8_u
                local.tee 2
                i32.le_u
                br_if 0 (;@6;)
                local.get 13
                i32.const 134217728
                i32.lt_u
                br_if 1 (;@5;)
              end
            end
            local.get 10
            i64.extend_i32_u
            local.set 7
          end
          local.get 1
          local.get 2
          i32.le_u
          br_if 0 (;@3;)
          i64.const -1
          local.get 12
          i64.extend_i32_u
          local.tee 9
          i64.shr_u
          local.tee 11
          local.get 7
          i64.lt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 2
            i64.extend_i32_u
            i64.const 255
            i64.and
            local.set 8
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 5
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 5
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 5
                i32.load8_u
                local.set 5
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 5
            end
            local.get 7
            local.get 9
            i64.shl
            local.get 8
            i64.or
            local.set 7
            local.get 1
            local.get 5
            i32.const 3569
            i32.add
            i32.load8_u
            local.tee 2
            i32.le_u
            br_if 1 (;@3;)
            local.get 7
            local.get 11
            i64.le_u
            br_if 0 (;@4;)
          end
        end
        local.get 1
        local.get 5
        i32.const 3569
        i32.add
        i32.load8_u
        i32.le_u
        br_if 0 (;@2;)
        loop ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 5
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 5
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 5
              i32.load8_u
              local.set 5
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 5
          end
          local.get 1
          local.get 5
          i32.const 3569
          i32.add
          i32.load8_u
          i32.gt_u
          br_if 0 (;@3;)
        end
        i32.const 0
        i32.const 68
        i32.store offset=4100
        local.get 6
        i32.const 0
        local.get 3
        i64.const 1
        i64.and
        i64.eqz
        select
        local.set 6
        local.get 3
        local.set 7
      end
      block ;; label = @2
        local.get 0
        i64.load offset=88
        i64.const 0
        i64.lt_s
        br_if 0 (;@2;)
        local.get 0
        local.get 0
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
      end
      block ;; label = @2
        local.get 7
        local.get 3
        i64.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.wrap_i64
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 6
          br_if 0 (;@3;)
          i32.const 0
          i32.const 68
          i32.store offset=4100
          local.get 3
          i64.const -1
          i64.add
          local.set 3
          br 2 (;@1;)
        end
        local.get 7
        local.get 3
        i64.le_u
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=4100
        br 1 (;@1;)
      end
      local.get 7
      local.get 6
      i64.extend_i32_s
      local.tee 3
      i64.xor
      local.get 3
      i64.sub
      local.set 3
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $strtol (;76;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store offset=40
    local.get 3
    local.get 0
    i32.store offset=4
    local.get 3
    i32.const -1
    i32.store offset=8
    local.get 3
    i64.const 0
    call $__shlim
    local.get 3
    local.get 2
    i32.const 1
    i64.const 2147483648
    call $__intscan
    local.set 4
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      local.get 3
      i32.load offset=4
      local.get 3
      i32.load offset=40
      i32.sub
      i32.add
      local.get 3
      i32.load offset=96
      i32.add
      i32.store
    end
    local.get 3
    i32.const 112
    i32.add
    global.set $__stack_pointer
    local.get 4
    i32.wrap_i64
  )
  (func $memchr (;77;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 2
    i32.const 0
    i32.ne
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 0
              i32.load8_u
              local.get 1
              i32.const 255
              i32.and
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.set 4
              local.get 2
              local.set 5
              br 3 (;@2;)
            end
            local.get 2
            i32.const -1
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -2
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -3
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 2
            i32.const -4
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            br 1 (;@3;)
          end
          local.get 2
          local.set 5
          local.get 0
          local.set 4
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 4
          i32.load8_u
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 0
          loop ;; label = @4
            i32.const 16843008
            local.get 4
            i32.load
            local.get 0
            i32.xor
            local.tee 2
            i32.sub
            local.get 2
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.ne
            br_if 2 (;@2;)
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 5
            i32.const -4
            i32.add
            local.tee 5
            i32.const 3
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 255
      i32.and
      local.set 2
      loop ;; label = @2
        block ;; label = @3
          local.get 4
          i32.load8_u
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          return
        end
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0
  )
  (func $memcmp (;78;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        loop ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3
  )
  (func $strdup (;79;) (type 4) (param i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      local.get 0
      call $strlen
      i32.const 1
      i32.add
      local.tee 1
      call $malloc
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 1
      memory.copy
    end
    local.get 2
  )
  (func $strlen (;80;) (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load8_u
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.sub
          return
        end
        local.get 0
        i32.const 1
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 2
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const -4
      i32.add
      local.set 2
      local.get 1
      i32.const -5
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        i32.const 16843008
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        i32.load
        local.tee 3
        i32.sub
        local.get 3
        i32.or
        i32.const -2139062144
        i32.and
        i32.const -2139062144
        i32.eq
        br_if 0 (;@2;)
      end
      loop ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.load8_u
        local.set 3
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 3
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub
  )
  (func $strnlen (;81;) (type 5) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.const 0
    local.get 1
    call $memchr
    local.tee 2
    local.get 0
    i32.sub
    local.get 1
    local.get 2
    select
  )
  (func $__multi3 (;82;) (type 17) (param i32 i64 i64 i64 i64)
    (local i64)
    local.get 0
    local.get 4
    local.get 1
    i64.mul
    local.get 2
    local.get 3
    i64.mul
    i64.add
    local.get 3
    i64.const 32
    i64.shr_u
    local.tee 2
    local.get 1
    i64.const 32
    i64.shr_u
    local.tee 4
    i64.mul
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.tee 3
    local.get 1
    i64.const 4294967295
    i64.and
    local.tee 1
    i64.mul
    local.tee 5
    i64.const 32
    i64.shr_u
    local.get 3
    local.get 4
    i64.mul
    i64.add
    local.tee 3
    i64.const 32
    i64.shr_u
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.get 2
    local.get 1
    i64.mul
    i64.add
    local.tee 1
    i64.const 32
    i64.shr_u
    i64.add
    i64.store offset=8
    local.get 0
    local.get 1
    i64.const 32
    i64.shl
    local.get 5
    i64.const 4294967295
    i64.and
    i64.or
    i64.store
  )
  (data $.rodata (;0;) (i32.const 1024) "-+   0X0x\00-0X+0X 0X-0x+0x 0x\00./shootout-ackermann.repeat.input\00./shootout-ackermann.n.input\00./shootout-ackermann.m.input\00nan\00inf\00NAN\00INF\00.\00(null)\00[ackermann] returned %d\0a\00[ackermann] running with M = %d and N = %d\0a\00Support for formatting long double values is currently disabled.\0aTo enable it, add -lc-printscan-long-double to the link command.\0a\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05\00\00\00\00\00\00\19\00\0b\00\19\19\19\00\00\00\00\05\00\00\00\00\00\00\09\00\00\00\00\0b\00\00\00\00\00\00\00\00\19\00\0a\0a\19\19\19\03\0a\07\00\01\1b\09\0b\18\00\00\09\06\0b\00\00\0b\00\06\19\00\00\00\19\19\19\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0e\00\00\00\00\00\00\00\00\19\00\0b\0d\19\19\19\00\0d\00\00\02\00\09\0e\00\00\00\09\00\0e\00\00\0e\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0c\00\00\00\00\00\00\00\00\00\00\00\13\00\00\00\00\13\00\00\00\00\09\0c\00\00\00\00\00\0c\00\00\0c\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\00\00\00\00\0f\00\00\00\04\0f\00\00\00\00\09\10\00\00\00\00\00\10\00\00\10\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\12\00\00\00\00\00\00\00\00\00\00\00\11\00\00\00\00\11\00\00\00\00\09\12\00\00\00\00\00\12\00\00\12\00\00\1a\00\00\00\1a\1a\1a\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\1a\00\00\00\1a\1a\1a\00\00\00\00\00\00\09\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\14\00\00\00\00\00\00\00\00\00\00\00\17\00\00\00\00\17\00\00\00\00\09\14\00\00\00\00\00\14\00\00\14\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\16\00\00\00\00\00\00\00\00\00\00\00\15\00\00\00\00\15\00\00\00\00\09\16\00\00\00\00\00\16\00\00\16\00\000123456789ABCDEF\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\01\02\03\04\05\06\07\08\09\ff\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f\10\11\12\13\14\15\16\17\18\19\1a\1b\1c\1d\1e\1f !\22#\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f\10\11\12\13\14\15\16\17\18\19\1a\1b\1c\1d\1e\1f !\22#\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\01\02\04\07\03\06\05\00")
  (data $.data (;1;) (i32.const 3840) "\01\00\00\00\00\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\04\00\00\00\05\00\00\00(\12\00\00\00\04\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\0a\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\08\0f\00\00\00\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\05\00\00\00T\16\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\ff\ff\ff\ff\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\80\0f\00\00\00\00\02\00")
  (@producers
    (language "C11" "")
    (processed-by "clang" "21.1.4-wasi-sdk (https://github.com/llvm/llvm-project 222fc11f2b8f25f6a0f4976272ef1bb7bf49521d)")
  )
  (@custom "target_features" (after data) "\09+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0eextended-const+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
)
