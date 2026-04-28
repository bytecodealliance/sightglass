(module
  ;; ===== Type Definitions =====

  (type $i32_array (array (mut i32)))
  (type $f64_array (array (mut f64)))

  (rec
    (type $payload (struct
      (field $left (mut (ref null $payload)))
      (field $right (mut (ref null $payload)))
      (field $array (ref null $i32_array))
    ))
  )

  (rec
    (type $node (struct
      (field $key (mut f64))
      (field $left (mut (ref null $node)))
      (field $right (mut (ref null $node)))
      (field $value (mut (ref null $payload)))
    ))
  )

  ;; ===== Imports =====

  (import "bench" "start" (func $bench_start))
  (import "bench" "end" (func $bench_end))

  (import "wasi_snapshot_preview1" "path_open"
    (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_read"
    (func $fd_read (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_close"
    (func $fd_close (param i32) (result i32)))

  ;; ===== Memory (for WASI I/O only) =====

  (memory (export "memory") 1)

  ;; Layout:
  ;;   0-7:     iovec (ptr i32 @ 0, len i32 @ 4)
  ;;   8-11:    nread/nwritten
  ;;   16-143:  I/O read buffer
  ;;   144-175: number formatting buffer
  ;;   256+:    string constants

  (data (i32.const 256) "default.input")            ;; 13 bytes @ 256
  (data (i32.const 280) "[splay] tree size: ")      ;; 19 bytes @ 280
  (data (i32.const 300) "\n")                        ;; 1 byte  @ 300
  (data (i32.const 304) "[splay] samples: ")         ;; 17 bytes @ 304
  (data (i32.const 324) "[splay] verified\n")        ;; 17 bytes @ 324

  ;; ===== Globals =====

  (global $splay_root (mut (ref null $node)) (ref.null none))
  (global $tree_size (mut i32) (i32.const 0))
  (global $rng_state (mut i64) (i64.const 0x12345678deadbeef))
  (global $sample_count (mut i32) (i32.const 0))

  (global $cfg_tree_size (mut i32) (i32.const 0))
  (global $cfg_modifications (mut i32) (i32.const 0))
  (global $cfg_payload_depth (mut i32) (i32.const 0))

  ;; ===== WASI Helpers =====

  ;; Print a string from linear memory to stdout.
  (func $print_str (param $ptr i32) (param $len i32)
    ;; Set up iovec at address 0: {ptr, len}
    (i32.store (i32.const 0) (local.get $ptr))
    (i32.store (i32.const 4) (local.get $len))
    ;; fd_write(fd=1, iovs=0, iovs_len=1, nwritten=8)
    (drop (call $fd_write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 8)))
  )

  ;; Print an i32 to stdout. Writes digits into buffer at 144-175, then prints.
  (func $print_i32 (param $val i32)
    (local $pos i32)
    (local $digit i32)
    (local $start i32)

    ;; Handle zero
    (if (i32.eqz (local.get $val))
      (then
        (i32.store8 (i32.const 144) (i32.const 48)) ;; '0'
        (call $print_str (i32.const 144) (i32.const 1))
        (return)
      )
    )

    ;; Write digits in reverse starting from position 175
    (local.set $pos (i32.const 175))
    (block $done
      (loop $digits
        (br_if $done (i32.eqz (local.get $val)))
        (local.set $digit (i32.rem_u (local.get $val) (i32.const 10)))
        (i32.store8 (local.get $pos) (i32.add (local.get $digit) (i32.const 48)))
        (local.set $val (i32.div_u (local.get $val) (i32.const 10)))
        (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
        (br $digits)
      )
    )

    ;; Print from pos+1 to 175 inclusive
    (local.set $start (i32.add (local.get $pos) (i32.const 1)))
    (call $print_str
      (local.get $start)
      (i32.sub (i32.const 176) (local.get $start))
    )
  )

  ;; Print newline
  (func $print_newline
    (call $print_str (i32.const 300) (i32.const 1))
  )

  ;; ===== Read Config =====

  ;; Open ./default.input via the preopened dir (fd 3), read, parse CSV.
  (func $read_config
    (local $fd i32)
    (local $nread i32)
    (local $pos i32)
    (local $val i32)
    (local $field i32)
    (local $ch i32)
    (local $end i32)

    ;; path_open(fd=3, dirflags=0, path=256, path_len=13,
    ;;           oflags=0, rights_base=fd_read, rights_inherit=0, fdflags=0, opened_fd=8)
    (if (call $path_open
          (i32.const 3)         ;; dirfd (preopened)
          (i32.const 0)         ;; dirflags
          (i32.const 256)       ;; path ptr ("default.input")
          (i32.const 13)        ;; path len
          (i32.const 0)         ;; oflags
          (i64.const 2)         ;; fs_rights_base: FD_READ
          (i64.const 0)         ;; fs_rights_inheriting
          (i32.const 0)         ;; fdflags
          (i32.const 12))       ;; opened_fd result ptr
      (then (unreachable))
    )
    (local.set $fd (i32.load (i32.const 12)))

    ;; Set up iovec to read into buffer at 16, length 128
    (i32.store (i32.const 0) (i32.const 16))
    (i32.store (i32.const 4) (i32.const 128))

    ;; fd_read(fd, iovs=0, iovs_len=1, nread=8)
    (if (call $fd_read (local.get $fd) (i32.const 0) (i32.const 1) (i32.const 8))
      (then (unreachable))
    )
    (local.set $nread (i32.load (i32.const 8)))
    (drop (call $fd_close (local.get $fd)))

    ;; Parse "8000,80,5\n" from buffer at 16
    (local.set $pos (i32.const 16))
    (local.set $end (i32.add (i32.const 16) (local.get $nread)))
    (local.set $val (i32.const 0))
    (local.set $field (i32.const 0))

    (block $parse_done
      (loop $parse
        (br_if $parse_done (i32.ge_u (local.get $pos) (local.get $end)))
        (local.set $ch (i32.load8_u (local.get $pos)))

        ;; comma or newline or end → store field
        (if (i32.or
              (i32.eq (local.get $ch) (i32.const 44))  ;; ','
              (i32.or
                (i32.eq (local.get $ch) (i32.const 10)) ;; '\n'
                (i32.eq (local.get $ch) (i32.const 13)) ;; '\r'
              )
            )
          (then
            ;; Store parsed value
            (if (i32.eqz (local.get $field))
              (then (global.set $cfg_tree_size (local.get $val)))
            )
            (if (i32.eq (local.get $field) (i32.const 1))
              (then (global.set $cfg_modifications (local.get $val)))
            )
            (if (i32.eq (local.get $field) (i32.const 2))
              (then (global.set $cfg_payload_depth (local.get $val)))
            )
            (local.set $field (i32.add (local.get $field) (i32.const 1)))
            (local.set $val (i32.const 0))
          )
          (else
            ;; Accumulate digit: val = val * 10 + (ch - '0')
            (local.set $val
              (i32.add
                (i32.mul (local.get $val) (i32.const 10))
                (i32.sub (local.get $ch) (i32.const 48))
              )
            )
          )
        )

        (local.set $pos (i32.add (local.get $pos) (i32.const 1)))
        (br $parse)
      )
    )

    ;; Handle last field if no trailing newline
    (if (i32.and
          (i32.gt_u (local.get $val) (i32.const 0))
          (i32.lt_u (local.get $field) (i32.const 3))
        )
      (then
        (if (i32.eqz (local.get $field))
          (then (global.set $cfg_tree_size (local.get $val)))
        )
        (if (i32.eq (local.get $field) (i32.const 1))
          (then (global.set $cfg_modifications (local.get $val)))
        )
        (if (i32.eq (local.get $field) (i32.const 2))
          (then (global.set $cfg_payload_depth (local.get $val)))
        )
      )
    )
  )

  ;; ===== PRNG =====

  ;; xorshift64* PRNG returning f64 in [0, 1)
  (func $rand (result f64)
    (local $s i64)
    (local $x i64)

    (local.set $s (global.get $rng_state))

    ;; state ^= state >> 12
    (local.set $s (i64.xor (local.get $s) (i64.shr_u (local.get $s) (i64.const 12))))
    ;; state ^= state << 25
    (local.set $s (i64.xor (local.get $s) (i64.shl (local.get $s) (i64.const 25))))
    ;; state ^= state >> 27
    (local.set $s (i64.xor (local.get $s) (i64.shr_u (local.get $s) (i64.const 27))))

    (global.set $rng_state (local.get $s))

    ;; x = state * 0x2545F4914F6CDD1D
    (local.set $x (i64.mul (local.get $s) (i64.const 0x2545F4914F6CDD1D)))

    ;; Convert to f64 in [0, 1): (x >> 11) / 2^53
    (f64.mul
      (f64.convert_i64_u (i64.shr_u (local.get $x) (i64.const 11)))
      (f64.const 1.1102230246251565e-16)
    )
  )

  ;; ===== Payload Tree =====

  ;; Generate a binary payload tree of the given depth.
  ;; Leaf nodes get an i32 array [0..9].
  (func $generate_payload (param $depth i32) (result (ref $payload))
    (local $arr (ref null $i32_array))

    (if (result (ref $payload)) (i32.eqz (local.get $depth))
      (then
        ;; Leaf: create array [0,1,2,3,4,5,6,7,8,9]
        (local.set $arr (array.new_default $i32_array (i32.const 10)))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 0) (i32.const 0))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 1) (i32.const 1))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 2) (i32.const 2))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 3) (i32.const 3))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 4) (i32.const 4))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 5) (i32.const 5))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 6) (i32.const 6))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 7) (i32.const 7))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 8) (i32.const 8))
        (array.set $i32_array (ref.as_non_null (local.get $arr)) (i32.const 9) (i32.const 9))
        (struct.new $payload
          (ref.null $payload)
          (ref.null $payload)
          (local.get $arr)
        )
      )
      (else
        ;; Internal: recurse left and right
        (struct.new $payload
          (call $generate_payload (i32.sub (local.get $depth) (i32.const 1)))
          (call $generate_payload (i32.sub (local.get $depth) (i32.const 1)))
          (ref.null $i32_array)
        )
      )
    )
  )

  ;; ===== Splay Tree Operations =====

  ;; Core splay operation: move the node with the given key (or the closest
  ;; node on the search path) to the root.
  ;; Port of SplayTree.prototype.splay_ from splay.js.
  (func $splay (param $key f64)
    (local $dummy (ref null $node))
    (local $l (ref null $node))
    (local $r (ref null $node))
    (local $current (ref null $node))
    (local $tmp (ref null $node))
    (local $current_nn (ref $node))
    (local $child (ref null $node))

    ;; if (this.isEmpty()) return;
    (if (ref.is_null (global.get $splay_root)) (then (return)))

    ;; dummy = left = right = new Node(null, null)
    (local.set $dummy
      (struct.new $node (f64.const 0) (ref.null none) (ref.null none) (ref.null none)))
    (local.set $l (local.get $dummy))
    (local.set $r (local.get $dummy))
    (local.set $current (global.get $splay_root))

    (block $break
      (loop $loop
        (local.set $current_nn (ref.as_non_null (local.get $current)))

        (if (f64.lt (local.get $key) (struct.get $node $key (local.get $current_nn)))
          (then
            ;; key < current.key
            (local.set $child (struct.get $node $left (local.get $current_nn)))
            (br_if $break (ref.is_null (local.get $child)))

            ;; if (key < current.left.key) → rotate right
            (if (f64.lt (local.get $key)
                  (struct.get $node $key (ref.as_non_null (local.get $child))))
              (then
                ;; tmp = current.left
                (local.set $tmp (local.get $child))
                ;; current.left = tmp.right
                (struct.set $node $left (local.get $current_nn)
                  (struct.get $node $right (ref.as_non_null (local.get $tmp))))
                ;; tmp.right = current
                (struct.set $node $right (ref.as_non_null (local.get $tmp)) (local.get $current_nn))
                ;; current = tmp
                (local.set $current (local.get $tmp))
                (local.set $current_nn (ref.as_non_null (local.get $current)))
                ;; if (!current.left) break
                (br_if $break (ref.is_null (struct.get $node $left (local.get $current_nn))))
              )
            )

            ;; Link right: right.left = current
            (struct.set $node $left (ref.as_non_null (local.get $r)) (local.get $current_nn))
            ;; right = current
            (local.set $r (local.get $current))
            ;; current = current.left
            (local.set $current (struct.get $node $left (local.get $current_nn)))
          )
          (else
            (if (f64.gt (local.get $key) (struct.get $node $key (local.get $current_nn)))
              (then
                ;; key > current.key
                (local.set $child (struct.get $node $right (local.get $current_nn)))
                (br_if $break (ref.is_null (local.get $child)))

                ;; if (key > current.right.key) → rotate left
                (if (f64.gt (local.get $key)
                      (struct.get $node $key (ref.as_non_null (local.get $child))))
                  (then
                    ;; tmp = current.right
                    (local.set $tmp (local.get $child))
                    ;; current.right = tmp.left
                    (struct.set $node $right (local.get $current_nn)
                      (struct.get $node $left (ref.as_non_null (local.get $tmp))))
                    ;; tmp.left = current
                    (struct.set $node $left (ref.as_non_null (local.get $tmp)) (local.get $current_nn))
                    ;; current = tmp
                    (local.set $current (local.get $tmp))
                    (local.set $current_nn (ref.as_non_null (local.get $current)))
                    ;; if (!current.right) break
                    (br_if $break (ref.is_null (struct.get $node $right (local.get $current_nn))))
                  )
                )

                ;; Link left: left.right = current
                (struct.set $node $right (ref.as_non_null (local.get $l)) (local.get $current_nn))
                ;; left = current
                (local.set $l (local.get $current))
                ;; current = current.right
                (local.set $current (struct.get $node $right (local.get $current_nn)))
              )
              (else
                ;; key == current.key
                (br $break)
              )
            )
          )
        )

        (br $loop)
      )
    )

    ;; Assemble
    (local.set $current_nn (ref.as_non_null (local.get $current)))

    ;; left.right = current.left
    (struct.set $node $right (ref.as_non_null (local.get $l))
      (struct.get $node $left (local.get $current_nn)))
    ;; right.left = current.right
    (struct.set $node $left (ref.as_non_null (local.get $r))
      (struct.get $node $right (local.get $current_nn)))
    ;; current.left = dummy.right
    (struct.set $node $left (local.get $current_nn)
      (struct.get $node $right (ref.as_non_null (local.get $dummy))))
    ;; current.right = dummy.left
    (struct.set $node $right (local.get $current_nn)
      (struct.get $node $left (ref.as_non_null (local.get $dummy))))
    ;; this.root_ = current
    (global.set $splay_root (local.get $current))
  )

  ;; Insert a key/value pair into the splay tree.
  (func $insert (param $key f64) (param $value (ref null $payload))
    (local $new_node (ref null $node))
    (local $root (ref null $node))
    (local $root_nn (ref $node))

    ;; if (this.isEmpty()) { this.root_ = new Node(key, value); return; }
    (if (ref.is_null (global.get $splay_root))
      (then
        (global.set $splay_root
          (struct.new $node (local.get $key) (ref.null none) (ref.null none) (local.get $value)))
        (global.set $tree_size (i32.add (global.get $tree_size) (i32.const 1)))
        (return)
      )
    )

    ;; this.splay_(key)
    (call $splay (local.get $key))
    (local.set $root (global.get $splay_root))
    (local.set $root_nn (ref.as_non_null (local.get $root)))

    ;; if (this.root_.key == key) return; (duplicate)
    (if (f64.eq (struct.get $node $key (local.get $root_nn)) (local.get $key))
      (then (return))
    )

    ;; var node = new Node(key, value)
    (local.set $new_node
      (struct.new $node (local.get $key) (ref.null none) (ref.null none) (local.get $value)))

    (if (f64.gt (local.get $key) (struct.get $node $key (local.get $root_nn)))
      (then
        ;; node.left = this.root_
        (struct.set $node $left (ref.as_non_null (local.get $new_node)) (local.get $root_nn))
        ;; node.right = this.root_.right
        (struct.set $node $right (ref.as_non_null (local.get $new_node))
          (struct.get $node $right (local.get $root_nn)))
        ;; this.root_.right = null
        (struct.set $node $right (local.get $root_nn) (ref.null none))
      )
      (else
        ;; node.right = this.root_
        (struct.set $node $right (ref.as_non_null (local.get $new_node)) (local.get $root_nn))
        ;; node.left = this.root_.left
        (struct.set $node $left (ref.as_non_null (local.get $new_node))
          (struct.get $node $left (local.get $root_nn)))
        ;; this.root_.left = null
        (struct.set $node $left (local.get $root_nn) (ref.null none))
      )
    )

    ;; this.root_ = node
    (global.set $splay_root (local.get $new_node))
    (global.set $tree_size (i32.add (global.get $tree_size) (i32.const 1)))
  )

  ;; Remove the node with the given key. Traps if not found.
  ;; Returns the removed node.
  (func $remove (param $key f64) (result (ref $node))
    (local $root (ref $node))
    (local $removed (ref $node))
    (local $right_child (ref null $node))

    ;; if (this.isEmpty()) throw
    (if (ref.is_null (global.get $splay_root)) (then (unreachable)))

    ;; this.splay_(key)
    (call $splay (local.get $key))
    (local.set $root (ref.as_non_null (global.get $splay_root)))

    ;; if (this.root_.key != key) throw
    (if (i32.eqz (f64.eq (struct.get $node $key (local.get $root)) (local.get $key)))
      (then (unreachable))
    )

    (local.set $removed (local.get $root))

    (if (ref.is_null (struct.get $node $left (local.get $root)))
      (then
        ;; this.root_ = this.root_.right
        (global.set $splay_root (struct.get $node $right (local.get $root)))
      )
      (else
        ;; var right = this.root_.right
        (local.set $right_child (struct.get $node $right (local.get $root)))
        ;; this.root_ = this.root_.left
        (global.set $splay_root (struct.get $node $left (local.get $root)))
        ;; this.splay_(key) — puts max of left subtree at root
        (call $splay (local.get $key))
        ;; this.root_.right = right
        (struct.set $node $right (ref.as_non_null (global.get $splay_root)) (local.get $right_child))
      )
    )

    (global.set $tree_size (i32.sub (global.get $tree_size) (i32.const 1)))
    (local.get $removed)
  )

  ;; Find the node with the given key. Returns null if not found.
  (func $find (param $key f64) (result (ref null $node))
    (local $root (ref $node))

    ;; if (this.isEmpty()) return null
    (if (ref.is_null (global.get $splay_root))
      (then (return (ref.null none)))
    )

    ;; this.splay_(key)
    (call $splay (local.get $key))
    (local.set $root (ref.as_non_null (global.get $splay_root)))

    ;; return this.root_.key == key ? this.root_ : null
    (if (result (ref null $node))
        (f64.eq (struct.get $node $key (local.get $root)) (local.get $key))
      (then (local.get $root))
      (else (ref.null none))
    )
  )

  ;; Find the node with the maximum key in the subtree rooted at the given node.
  (func $find_max (param $start (ref $node)) (result (ref $node))
    (local $current (ref $node))
    (local $right_child (ref null $node))

    (local.set $current (local.get $start))
    (block $done
      (loop $walk
        (local.set $right_child (struct.get $node $right (local.get $current)))
        (br_if $done (ref.is_null (local.get $right_child)))
        (local.set $current (ref.as_non_null (local.get $right_child)))
        (br $walk)
      )
    )
    (local.get $current)
  )

  ;; Find the node with the greatest key less than the given key.
  (func $find_greatest_less_than (param $key f64) (result (ref null $node))
    (local $root (ref $node))
    (local $left_child (ref null $node))

    ;; if (this.isEmpty()) return null
    (if (ref.is_null (global.get $splay_root))
      (then (return (ref.null none)))
    )

    ;; this.splay_(key)
    (call $splay (local.get $key))
    (local.set $root (ref.as_non_null (global.get $splay_root)))

    ;; if (this.root_.key < key) return this.root_
    (if (f64.lt (struct.get $node $key (local.get $root)) (local.get $key))
      (then (return (local.get $root)))
    )

    ;; else if (this.root_.left) return this.findMax(this.root_.left)
    (local.set $left_child (struct.get $node $left (local.get $root)))
    (if (result (ref null $node)) (i32.eqz (ref.is_null (local.get $left_child)))
      (then (call $find_max (ref.as_non_null (local.get $left_child))))
      (else (ref.null none))
    )
  )

  ;; In-order traversal collecting keys into an f64 array.
  ;; Ports Node.prototype.traverse_.
  (func $traverse (param $current (ref null $node)) (param $keys (ref $f64_array)) (param $idx i32) (result i32)
    (local $current_nn (ref $node))
    (local $left_child (ref null $node))

    (block $break
      (loop $loop
        (br_if $break (ref.is_null (local.get $current)))
        (local.set $current_nn (ref.as_non_null (local.get $current)))

        ;; var left = current.left
        (local.set $left_child (struct.get $node $left (local.get $current_nn)))

        ;; if (left) left.traverse_(f)
        (if (i32.eqz (ref.is_null (local.get $left_child)))
          (then
            (local.set $idx
              (call $traverse (local.get $left_child) (local.get $keys) (local.get $idx)))
          )
        )

        ;; f(current) → keys[idx] = current.key; idx++
        (array.set $f64_array (local.get $keys) (local.get $idx)
          (struct.get $node $key (local.get $current_nn)))
        (local.set $idx (i32.add (local.get $idx) (i32.const 1)))

        ;; current = current.right
        (local.set $current (struct.get $node $right (local.get $current_nn)))
        (br $loop)
      )
    )

    (local.get $idx)
  )

  ;; Export all keys as a sorted f64 array.
  (func $export_keys (result (ref $f64_array))
    (local $keys (ref null $f64_array))

    ;; if (this.isEmpty()) return []
    (if (ref.is_null (global.get $splay_root))
      (then
        (return (array.new_default $f64_array (i32.const 0)))
      )
    )

    (local.set $keys (array.new_default $f64_array (global.get $tree_size)))
    (drop (call $traverse
      (global.get $splay_root)
      (ref.as_non_null (local.get $keys))
      (i32.const 0)))

    (ref.as_non_null (local.get $keys))
  )

  ;; ===== Benchmark Logic =====

  ;; Insert a new node with a unique random key and a payload tree.
  ;; Returns the key.
  (func $insert_new_node (result f64)
    (local $key f64)
    (local $payload_val (ref null $payload))

    ;; Generate a unique key
    (block $unique
      (loop $retry
        (local.set $key (call $rand))
        (br_if $retry (i32.eqz (ref.is_null (call $find (local.get $key)))))
      )
    )

    ;; Generate payload tree
    (local.set $payload_val (call $generate_payload (global.get $cfg_payload_depth)))

    ;; Insert into the splay tree
    (call $insert (local.get $key) (local.get $payload_val))

    (local.get $key)
  )

  ;; Build the initial splay tree.
  (func $splay_setup
    (local $i i32)

    (global.set $splay_root (ref.null none))
    (global.set $tree_size (i32.const 0))
    (global.set $sample_count (i32.const 0))

    (local.set $i (i32.const 0))
    (block $done
      (loop $build
        (br_if $done (i32.ge_u (local.get $i) (global.get $cfg_tree_size)))

        (drop (call $insert_new_node))

        ;; SplayUpdateStats every 20 inserts: (i+1) % 20 == 19
        (if (i32.eq
              (i32.rem_u (i32.add (local.get $i) (i32.const 1)) (i32.const 20))
              (i32.const 19))
          (then
            (global.set $sample_count (i32.add (global.get $sample_count) (i32.const 1)))
          )
        )

        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $build)
      )
    )
  )

  ;; One iteration of the benchmark: replace $cfg_modifications nodes.
  (func $splay_run
    (local $i i32)
    (local $key f64)
    (local $greatest (ref null $node))

    (local.set $i (i32.const 0))
    (block $done
      (loop $modify
        (br_if $done (i32.ge_u (local.get $i) (global.get $cfg_modifications)))

        ;; var key = InsertNewNode()
        (local.set $key (call $insert_new_node))

        ;; var greatest = splayTree.findGreatestLessThan(key)
        (local.set $greatest (call $find_greatest_less_than (local.get $key)))

        ;; if (greatest == null) splayTree.remove(key)
        ;; else splayTree.remove(greatest.key)
        (if (ref.is_null (local.get $greatest))
          (then (drop (call $remove (local.get $key))))
          (else (drop (call $remove
            (struct.get $node $key (ref.as_non_null (local.get $greatest))))))
        )

        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $modify)
      )
    )

    ;; SplayUpdateStats (count sample)
    (global.set $sample_count (i32.add (global.get $sample_count) (i32.const 1)))
  )

  ;; Verify the splay tree: check size and sorted unique keys.
  (func $splay_teardown
    (local $keys (ref null $f64_array))
    (local $length i32)
    (local $i i32)

    (local.set $keys (call $export_keys))
    (local.set $length (array.len (ref.as_non_null (local.get $keys))))

    ;; Verify size
    (if (i32.ne (local.get $length) (global.get $cfg_tree_size))
      (then (unreachable))
    )

    ;; Verify sorted and unique
    (local.set $i (i32.const 0))
    (block $verified
      (loop $check
        (br_if $verified
          (i32.ge_u (local.get $i) (i32.sub (local.get $length) (i32.const 1))))

        (if (i32.eqz (f64.lt
              (array.get $f64_array (ref.as_non_null (local.get $keys)) (local.get $i))
              (array.get $f64_array (ref.as_non_null (local.get $keys))
                (i32.add (local.get $i) (i32.const 1)))))
          (then (unreachable))
        )

        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $check)
      )
    )

    ;; Clear tree for GC
    (global.set $splay_root (ref.null none))
    (global.set $tree_size (i32.const 0))
  )

  ;; ===== Entry Point =====

  (func $start (export "_start")
    (local $i i32)

    ;; 1. Read config
    (call $read_config)

    ;; 2. Setup splay tree
    (call $splay_setup)

    ;; 3. Benchmark start
    (call $bench_start)

    ;; 4. Run 50 iterations
    (local.set $i (i32.const 0))
    (block $done
      (loop $iter
        (br_if $done (i32.ge_u (local.get $i) (i32.const 50)))
        (call $splay_run)
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $iter)
      )
    )

    ;; 5. Benchmark end
    (call $bench_end)

    ;; 6. Teardown and verify
    (call $splay_teardown)

    ;; 7. Print stats
    ;; "[splay] tree size: "
    (call $print_str (i32.const 280) (i32.const 19))
    (call $print_i32 (global.get $cfg_tree_size))
    (call $print_newline)
    ;; "[splay] samples: "
    (call $print_str (i32.const 304) (i32.const 17))
    (call $print_i32 (global.get $sample_count))
    (call $print_newline)
    ;; "[splay] verified\n"
    (call $print_str (i32.const 324) (i32.const 17))
  )
)
