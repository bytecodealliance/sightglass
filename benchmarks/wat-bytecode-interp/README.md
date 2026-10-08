# `wat-bytecode-interp`

A bytecode interpreter hand-written in the WebAssembly text format as a
component. It runs a recursive Fibonacci program, read from `default.input`,
and prints `fib(N) = X`.

It exercises the classic interpreter shape: a `loop` + `br_table` dispatch over
opcodes in linear memory, with the operand stack, frame pointer, and control
stack all held in memory. Recursive `fib` makes it heavy on `call`/`ret` and on
data-dependent conditional branches.

The component imports `bench` and the WASIp2 filesystem/stdout interfaces it
needs, using the same plumbing as [`cm-online-stats`](../cm-online-stats/).
`wasm-tools wat2wasm` assembles it via the shared `Dockerfile.wasm-tools`.

## Input format

`default.input` is whitespace-separated decimal integers. `#` starts a comment
that runs to the end of the line. The first two integers are `N REPS`, and the
rest are the bytecode. The interpreter runs the program `REPS` times. Each run
starts at word 0 with `N` as the only operand and as local 0 of the outermost
frame. A copy of the input is compiled in as a fallback for when the file is
missing.

Each instruction is one opcode word followed by its immediates. Jump and call
targets are word indices.

| op | name | immediates | effect |
| --- | --- | --- | --- |
| 0 | `halt` | | stop; the result is the top of stack |
| 1 | `push` | `k` | push `k` |
| 2 | `load` | `i` | push local `i` |
| 3 | `store` | `i` | pop into local `i` |
| 4 | `add` | | `a b -> a+b` |
| 5 | `sub` | | `a b -> a-b` |
| 6 | `lt` | | `a b -> a<b` (signed) |
| 7 | `jz` | `t` | pop; jump to `t` if zero |
| 8 | `jmp` | `t` | jump to `t` |
| 9 | `call` | `t n` | call `t`; the top `n` operands become locals `0..n` |
| 10 | `ret` | | pop the frame and its locals, push the return value |
| 11 | `dup` | | `a -> a a` |
| 12 | `drop` | | `a ->` |
| 13 | `mul` | | `a b -> a*b` |
| 14 | `eq` | | `a b -> a==b` |
| 15 | `jnz` | `t` | pop; jump to `t` if nonzero |

Values are `i32` and arithmetic wraps. Malformed input, invalid opcodes, and
overflow of the operand stack (16K values), the control stack (8K calls), or
the bytecode area (15K words) trap.

## Workload

`N REPS` is `25 1`, about 103M Wasm instructions.

## License

Original work, part of sightglass, available under the same licenses as the rest
of the repository (Apache-2.0 WITH LLVM-exception, or MIT).
