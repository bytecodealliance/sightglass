#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include "include/wasm.h"

extern "C" {

    typedef struct state_t
    {
        wasm_engine_t* engine;
        wasm_store_t* store;
        wasm_module_t* module;
        wasm_instance_t* instance;
    } state_t;

#define RETURN_ON_INVALID_POINTER(pointer, name) \
    if (!pointer)                                \
    {                                            \
        printf("Invalid pointer: " #name "\n");  \
        return 1;                                \
    }

#define EXIT_ON_INVALID_POINTER(pointer, name)  \
    if (!pointer)                               \
    {                                           \
        printf("Invalid pointer: " #name "\n"); \
        exit(1);                                \
    }

    void* wasm_bench_create()
    {
        state_t* state = (state_t*)calloc(1, sizeof(state_t)); // TODO initialize to all 0s
        state->engine = wasm_engine_new();
        EXIT_ON_INVALID_POINTER(state->engine, engine);
        state->store = wasm_store_new(state->engine);
        EXIT_ON_INVALID_POINTER(state->store, store);
        return state;
    }

    void wasm_bench_free(void* state_)
    {
        state_t* state = (state_t*)state_;
        wasm_module_delete(state->module);
        wasm_instance_delete(state->instance);
        wasm_store_delete(state->store);
        wasm_engine_delete(state->engine);
    }

    int wasm_bench_compile(void* state_, char* wasm_bytes, size_t wasm_bytes_length)
    {
        state_t* state = (state_t*)state_;
        RETURN_ON_INVALID_POINTER(state, state);
        wasm_byte_vec_t binary = { .data = wasm_bytes, .size = wasm_bytes_length };
        RETURN_ON_INVALID_POINTER(state->store, store);
        state->module = wasm_module_new(state->store, &binary);
        RETURN_ON_INVALID_POINTER(state->module, module);
        return 0;
    }

    int wasm_bench_instantiate(void* state_, void (*bench_start)(), void (*bench_end)())
    {
        state_t* state = (state_t*)state_;
        RETURN_ON_INVALID_POINTER(state, state);
        RETURN_ON_INVALID_POINTER(state->store, store);
        wasm_functype_t* bench_fn_type = wasm_functype_new_0_0();
        wasm_func_t* bench_start_fn = wasm_func_new(state->store, bench_fn_type,
            (wasm_func_callback_t)bench_start);
        wasm_func_t* bench_end_fn = wasm_func_new(state->store, bench_fn_type, (wasm_func_callback_t)bench_end);
        wasm_functype_delete(bench_fn_type);

        wasm_extern_t* externs[] = { wasm_func_as_extern(bench_start_fn), wasm_func_as_extern(bench_end_fn) };
        wasm_extern_vec_t imports = WASM_ARRAY_VEC(externs);
        RETURN_ON_INVALID_POINTER(state->module, module);
        state->instance = wasm_instance_new(state->store, state->module, &imports, NULL);
        RETURN_ON_INVALID_POINTER(state->instance, instance);
        wasm_func_delete(bench_start_fn);
        wasm_func_delete(bench_end_fn);

        return 0;
    }

    int wasm_bench_execute(void* state_)
    {
        state_t* state = (state_t*)state_;
        RETURN_ON_INVALID_POINTER(state, state);
        RETURN_ON_INVALID_POINTER(state->instance, instance);

        wasm_extern_vec_t exports;
        wasm_instance_exports(state->instance, &exports);
        if (exports.size == 0)
        {
            printf("> Error accessing exports!\n");
            return 1;
        }

        // TODO check that this is the _start function.
        const wasm_func_t* run_func = wasm_extern_as_func(exports.data[0]);
        RETURN_ON_INVALID_POINTER(run_func, run_func);

        wasm_val_vec_t args = WASM_EMPTY_VEC;
        wasm_val_vec_t results = WASM_EMPTY_VEC;
        if (wasm_func_call(run_func, &args, &results))
        {
            printf("> Error calling function!\n");
            return 1;
        }

        wasm_extern_vec_delete(&exports);
        return 0;
    }
} // extern "C"
