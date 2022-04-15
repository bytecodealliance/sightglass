# This Makefile builds both the V8 components and the bench API wrapper for Sightglass.
all: v8-checkout test

######### V8 ##########

# In order to build the `libwee8` library, we must perform the following steps:
# - `make v8-checkout` will retrieve the V8 repository (borrowing much the same logic as
#   https://github.com/WebAssembly/wasm-c-api)
# - `make wee8` will build the `libwee8.a` library inside the v8 tree (see See
#   https://docs.google.com/document/d/1oFPHyNb_eXg6NzrE6xJDNPdJrHMZvx0LqsD6wpbd9vY/edit#)

V8_VERSION = branch-heads/9.9
V8_ARCH = x64
V8_MODE = release
V8_BUILD = ${V8_ARCH}.${V8_MODE}
V8_OUT = ${V8_V8}/out.gn/${V8_BUILD}
V8_DIR = v8
V8_DEPOT_TOOLS = ${V8_DIR}/depot_tools
V8_V8 = ${V8_DIR}/v8
V8_PATH = $(abspath ${V8_DEPOT_TOOLS}):${PATH}
V8_LIBWEE8 = ${V8_V8}/out/wee8/obj/libwee8.a

.PHONY: v8-checkout
v8-checkout: ${V8_DEPOT_TOOLS} ${V8_V8}
	(cd ${V8_V8}; git checkout -f main)
	(cd ${V8_V8}; git pull)
	(cd ${V8_V8}; git checkout ${V8_VERSION})
	(cd ${V8_V8}; PATH=${V8_PATH} gclient sync)
	mkdir -p ${V8_OUT}
	echo >${V8_OUT}/version ${V8_VERSION}
${V8_DEPOT_TOOLS}:
	mkdir -p ${V8_DIR}
	(cd ${V8_DIR}; git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git --depth 1)
${V8_V8}:
	mkdir -p ${V8_DIR}
	(cd ${V8_DIR}; PATH=${V8_PATH} fetch --no-history v8)
	(cd ${V8_V8}; git checkout ${V8_VERSION})

.PHONY: wee8
wee8: ${V8_LIBWEE8}
${V8_LIBWEE8}:
	mkdir -p ${V8_V8}/out/wee8
	cp args.gn ${V8_V8}/out/wee8/
	(cd ${V8_V8}; PATH=${V8_PATH} gn gen out/wee8)
	(cd ${V8_V8}; PATH=${V8_PATH} autoninja -C out/wee8 wee8)

.PHONY: v8-clean
v8-clean:
	rm -rf v8



######### BENCH-API ##########

# The shared library that Sightglass expects must conform to a specific interface (see
# `src/bench-api.h`). We interact with `libwee8.a` in `src/bench-state.cc`. To build and test this
# functionality, use `make test`.

# This Makefile setup roughly follows https://stackoverflow.com/a/2481326.
CXXFLAGS=-g -O0 -fPIC -I ${V8_V8}/third_party/wasm-api
WASM_HH=${V8_V8}/third_party/wasm-api/wasm.hh
SRC_DIR=src
TEST_DIR=test
BUILD_DIR=build
SRCS=src/bench-api.cc src/bench-state.cc src/link.cc src/wasi.cc
OBJS=$(patsubst $(SRC_DIR)/%.cc,$(BUILD_DIR)/%.o,$(SRCS))
LDLIBS=$(V8_LIBWEE8) -ldl -pthread

# Create the build directory, if necessary.
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Built-in rules to compile the various source files.
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cc
	$(CXX) $(CXXFLAGS) -c $< -o $@
$(BUILD_DIR)/%.o: $(TEST_DIR)/%.c
	$(CC) $(CFLAGS) -I $(SRC_DIR) -c $< -o $@

# Define the dependencies for each object.
$(BUILD_DIR)/bench-api.o: $(SRC_DIR)/bench-api.cc $(SRC_DIR)/bench-api.h $(WASM_HH)
$(BUILD_DIR)/bench-state.o: $(SRC_DIR)/bench-state.cc $(SRC_DIR)/bench-state.hh $(SRC_DIR)/bench-api.h $(WASM_HH)
$(BUILD_DIR)/link.o: $(SRC_DIR)/link.cc $(SRC_DIR)/link.hh $(SRC_DIR)/bench-state.hh $(SRC_DIR)/wasi.hh $(WASM_HH)
$(BUILD_DIR)/wasi.o: $(SRC_DIR)/wasi.cc $(SRC_DIR)/wasi.hh $(WASM_HH)
$(BUILD_DIR)/main.o: $(TEST_DIR)/main.c $(SRC_DIR)/bench-api.h

# Create a test target for building and running the test file with `make test`.
.PHONY: test
test: $(BUILD_DIR)/test
	$< ../../benchmarks-next/noop/benchmark.wasm
$(BUILD_DIR)/test: $(BUILD_DIR) $(OBJS) $(BUILD_DIR)/main.o
	$(CXX) -g $(LDFLAGS) -o $@ $(OBJS) $(BUILD_DIR)/main.o $(LDLIBS)

.PHONY: test
clean:
	rm -rf build
