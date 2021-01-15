(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i64 i32) (result i64)))
  (import "bench" "start" (func $bench_start (type 1)))
  (import "bench" "end" (func $bench_end (type 1)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type 0)))
  (func $__wasm_call_ctors (type 1)
    nop)
  (func $__original_main (type 4) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 192
    i32.sub
    local.tee 0
    global.set 0
    call $bench_start
    block  ;; label = @1
      loop  ;; label = @2
        local.get 0
        i32.const 16
        i32.add
        i32.const 1024
        i32.const 121
        call $memcpy
        drop
        local.get 0
        local.get 0
        i32.const 16
        i32.add
        i32.store offset=12
        local.get 0
        i32.const 16
        i32.add
        i32.const 1148
        i32.load
        call_indirect (type 0)
        local.get 0
        local.get 0
        i32.load offset=12
        local.get 0
        i32.const 144
        i32.add
        local.get 0
        i32.const 140
        i32.add
        call $minicsv_parse_line
        i32.store offset=12
        local.get 0
        i32.load offset=140
        local.tee 5
        i32.const 11
        i32.ge_u
        br_if 1 (;@1;)
        i32.const 0
        local.set 4
        local.get 5
        if  ;; label = @3
          loop  ;; label = @4
            local.get 0
            i32.const 144
            i32.add
            local.get 4
            i32.const 2
            i32.shl
            i32.add
            local.tee 6
            i32.load
            local.tee 1
            local.set 2
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 3
                i32.const 255
                i32.and
                call $isspace
                i32.eqz
                if  ;; label = @7
                  local.get 1
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 6
                local.get 1
                i32.const 1
                i32.add
                local.tee 2
                i32.store
                local.get 1
                i32.load8_u offset=1
                local.set 3
                local.get 2
                local.set 1
                local.get 3
                br_if 0 (;@6;)
              end
            end
            block  ;; label = @5
              local.get 2
              call $strlen
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 2
                local.get 1
                i32.const -1
                i32.add
                local.tee 1
                i32.add
                local.tee 3
                i32.load8_u
                call $isspace
                i32.eqz
                br_if 1 (;@5;)
                local.get 3
                i32.const 0
                i32.store8
                local.get 1
                br_if 0 (;@6;)
              end
            end
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            local.get 5
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 0
        i32.const 144
        i32.add
        i32.const 1148
        i32.load
        call_indirect (type 0)
        local.get 0
        local.get 0
        i32.load offset=12
        local.get 0
        i32.const 144
        i32.add
        local.get 0
        i32.const 140
        i32.add
        call $minicsv_parse_line
        i32.store offset=12
        local.get 0
        i32.load offset=140
        local.tee 5
        i32.const 11
        i32.ge_u
        br_if 1 (;@1;)
        i32.const 0
        local.set 4
        local.get 5
        if  ;; label = @3
          loop  ;; label = @4
            local.get 0
            i32.const 144
            i32.add
            local.get 4
            i32.const 2
            i32.shl
            i32.add
            local.tee 6
            i32.load
            local.tee 1
            local.set 2
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 3
                i32.const 255
                i32.and
                call $isspace
                i32.eqz
                if  ;; label = @7
                  local.get 1
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 6
                local.get 1
                i32.const 1
                i32.add
                local.tee 2
                i32.store
                local.get 1
                i32.load8_u offset=1
                local.set 3
                local.get 2
                local.set 1
                local.get 3
                br_if 0 (;@6;)
              end
            end
            block  ;; label = @5
              local.get 2
              call $strlen
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 2
                local.get 1
                i32.const -1
                i32.add
                local.tee 1
                i32.add
                local.tee 3
                i32.load8_u
                call $isspace
                i32.eqz
                br_if 1 (;@5;)
                local.get 3
                i32.const 0
                i32.store8
                local.get 1
                br_if 0 (;@6;)
              end
            end
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            local.get 5
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 0
        i32.const 140
        i32.add
        i32.const 1148
        i32.load
        call_indirect (type 0)
        local.get 0
        i32.const 12
        i32.add
        i32.const 1148
        i32.load
        call_indirect (type 0)
        local.get 7
        i32.const 1
        i32.add
        local.tee 7
        i32.const 1000000
        i32.ne
        br_if 0 (;@2;)
      end
      call $bench_end
      local.get 0
      i32.const 192
      i32.add
      global.set 0
      i32.const 0
      return
    end
    call $abort
    unreachable)
  (func $minicsv_parse_line (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 2
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          i32.eqz
          if  ;; label = @4
            local.get 0
            local.set 5
            br 1 (;@3;)
          end
          local.get 0
          local.set 5
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  br_table 0 (;@7;) 1 (;@6;) 6 (;@1;) 2 (;@5;)
                end
                i32.const 0
                local.set 3
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.const 24
                          i32.shl
                          i32.const 24
                          i32.shr_s
                          i32.const -10
                          i32.add
                          br_table 0 (;@11;) 4 (;@7;) 4 (;@7;) 3 (;@8;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 2 (;@9;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 1 (;@10;) 4 (;@7;)
                        end
                        i32.const 2
                        local.set 3
                      end
                      local.get 0
                      i32.const 0
                      i32.store8
                      local.get 2
                      i32.load
                      local.tee 4
                      i32.const 9
                      i32.le_u
                      if  ;; label = @10
                        local.get 1
                        local.get 4
                        i32.const 2
                        i32.shl
                        i32.add
                        local.get 5
                        i32.store
                      end
                      local.get 2
                      local.get 4
                      i32.const 1
                      i32.add
                      i32.store
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 5
                      local.set 0
                      br 4 (;@5;)
                    end
                    i32.const 1
                    local.set 3
                    local.get 0
                    i32.const 1
                    i32.add
                    local.tee 5
                    local.set 0
                    br 3 (;@5;)
                  end
                  local.get 0
                  i32.const 0
                  i32.store8
                end
                local.get 0
                i32.const 1
                i32.add
                local.set 0
                br 1 (;@5;)
              end
              i32.const 1
              local.set 3
              block  ;; label = @6
                local.get 4
                i32.const 255
                i32.and
                i32.const 34
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.load8_u offset=1
                i32.const 34
                i32.eq
                if  ;; label = @7
                  local.get 0
                  local.get 0
                  i32.const 1
                  i32.add
                  local.get 0
                  call $strlen
                  call $memmove
                  drop
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 3
                local.get 0
                i32.const 0
                i32.store8
              end
              local.get 0
              i32.const 1
              i32.add
              local.set 0
            end
            local.get 0
            i32.load8_u
            local.tee 4
            br_if 0 (;@4;)
          end
          local.get 3
          br_if 2 (;@1;)
          local.get 2
          i32.load
          local.tee 3
          i32.const 9
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 1
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        local.get 5
        i32.store
      end
      local.get 2
      local.get 3
      i32.const 1
      i32.add
      i32.store
    end
    local.get 0)
  (func $_black_box (type 0) (param i32)
    nop)
  (func $_start (type 1)
    call $__wasm_call_ctors
    call $__original_main
    call $exit
    unreachable)
  (func $isspace (type 2) (param i32) (result i32)
    local.get 0
    i32.const 32
    i32.eq
    local.get 0
    i32.const -9
    i32.add
    i32.const 5
    i32.lt_u
    i32.or)
  (func $__errno_location (type 4) (result i32)
    i32.const 1152)
  (func $_Exit (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $abort (type 1)
    i32.const 1
    call $_Exit
    unreachable)
  (func $emscripten_memcpy_big (type 3) (param i32 i32 i32) (result i32)
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
  (func $memcpy (type 3) (param i32 i32 i32) (result i32)
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
  (func $memmove (type 3) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.sub
      local.get 2
      i32.sub
      i32.const 0
      local.get 2
      i32.const 1
      i32.shl
      i32.sub
      i32.le_u
      if  ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        call $memcpy
        return
      end
      local.get 0
      local.get 1
      i32.xor
      i32.const 3
      i32.and
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 1
          i32.lt_u
          if  ;; label = @4
            local.get 3
            if  ;; label = @5
              local.get 0
              local.set 3
              br 3 (;@2;)
            end
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            if  ;; label = @5
              local.get 0
              local.set 3
              br 2 (;@3;)
            end
            local.get 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 3
              local.get 1
              i32.load8_u
              i32.store8
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 2
              i32.const -1
              i32.add
              local.set 2
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@5;)
            end
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.add
            i32.const 3
            i32.and
            if  ;; label = @5
              loop  ;; label = @6
                local.get 2
                i32.eqz
                br_if 5 (;@1;)
                local.get 0
                local.get 2
                i32.const -1
                i32.add
                local.tee 2
                i32.add
                local.tee 3
                local.get 1
                local.get 2
                i32.add
                i32.load8_u
                i32.store8
                local.get 3
                i32.const 3
                i32.and
                br_if 0 (;@6;)
              end
            end
            local.get 2
            i32.const 3
            i32.le_u
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 0
              local.get 2
              i32.const -4
              i32.add
              local.tee 2
              i32.add
              local.get 1
              local.get 2
              i32.add
              i32.load
              i32.store
              local.get 2
              i32.const 3
              i32.gt_u
              br_if 0 (;@5;)
            end
          end
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          loop  ;; label = @4
            local.get 0
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            i32.add
            local.get 1
            local.get 2
            i32.add
            i32.load8_u
            i32.store8
            local.get 2
            br_if 0 (;@4;)
          end
          br 2 (;@1;)
        end
        local.get 2
        i32.const 3
        i32.le_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 3
          i32.const 4
          i32.add
          local.set 3
          local.get 2
          i32.const -4
          i32.add
          local.tee 2
          i32.const 3
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $strlen (type 2) (param i32) (result i32)
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
  (func $stackRestore (type 0) (param i32)
    local.get 0
    global.set 0)
  (func $stackAlloc (type 2) (param i32) (result i32)
    (local i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 1
    global.set 0
    local.get 1)
  (func $__lockfile (type 2) (param i32) (result i32)
    i32.const 1)
  (func $__unlockfile (type 0) (param i32)
    nop)
  (func $__lock (type 0) (param i32)
    nop)
  (func $__unlock (type 0) (param i32)
    nop)
  (func $__ofl_lock (type 4) (result i32)
    i32.const 1156
    call $__lock
    i32.const 1164)
  (func $__ofl_unlock (type 1)
    i32.const 1156
    call $__unlock)
  (func $fflush (type 2) (param i32) (result i32)
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
      i32.const 1168
      i32.load
      if  ;; label = @2
        i32.const 1168
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
  (func $__fflush_unlocked (type 2) (param i32) (result i32)
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
      call_indirect (type 3)
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
  (global (;0;) (mut i32) (i32.const 5244064))
  (global (;1;) i32 (i32.const 1172))
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
  (data (;0;) (i32.const 1024) "first,line,has,\22comas,\22\22escaped\22\22 characters\22,and,\22multiples\0d\0alines\22\0d\0asecond,line,\22  has  \22,,empty,,,,columns\0d\0aremainder")
  (data (;1;) (i32.const 1148) "\01"))
