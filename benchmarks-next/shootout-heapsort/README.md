Note that this benchmark still is still built with emscripten's standalone mode,
not WASI. That means its inputs aren't read from the file system like they
should be and like other benchmark programs' inputs are. When compiled with
WASI, this program traps with a memory access out of bounds error. We should fix
this or remove this test.
