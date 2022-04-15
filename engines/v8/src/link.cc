
#include "link.hh"
#include <iostream>
#include <map>
#include "wasi.hh"

std::vector<ImportName> list_imports(const wasm::Module* module) {
  auto import_types = module->imports();
  auto import_names = std::vector<ImportName>();
  for (size_t i = 0; i < import_types.size(); ++i) {
    wasm::ImportType* type = import_types[i].get();
    auto module_str = std::string(type->module().get(), type->module().size());
    auto name_str = std::string(type->name().get(), type->name().size());
    // std::cerr << "Found import: " << module_str << " " << name_str <<
    // std::endl;
    import_names.push_back(ImportName(module_str, name_str));
  }
  // std::cerr << "Number of imports: " << import_names.size() << std::endl;
  return import_names;
}

auto bench_start(void* env, const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  auto timer = *reinterpret_cast<Timer*>(env);
  std::cout << "bench_start" << std::endl;
  timer.start();
  return nullptr;
}

auto bench_end(void* env, const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  auto timer = *reinterpret_cast<Timer*>(env);
  timer.end();
  std::cout << "bench_end" << std::endl;
  return nullptr;
}

std::vector<const wasm::Extern*> link(wasm::Store* store,
                                      const wasm::Module* module,
                                      Timer* timer) {
  // Set up import types.
  auto fn_type_none_none = wasm::FuncType::make(
      wasm::ownvec<wasm::ValType>::make(), wasm::ownvec<wasm::ValType>::make());
  auto fn_type_i32_none = wasm::FuncType::make(
      wasm::ownvec<wasm::ValType>::make(wasm::ValType::make(wasm::I32)),
      wasm::ownvec<wasm::ValType>::make());
  auto fn_type_4xi32_i32 = wasm::FuncType::make(
      wasm::ownvec<wasm::ValType>::make(
          wasm::ValType::make(wasm::I32), wasm::ValType::make(wasm::I32),
          wasm::ValType::make(wasm::I32), wasm::ValType::make(wasm::I32)),
      wasm::ownvec<wasm::ValType>::make(wasm::ValType::make(wasm::I32)));

  // Set up available functions to link to.
  std::map<ImportName, wasm::own<wasm::Func>> available_functions;
  available_functions[ImportName("bench", "start")] = std::move(
      wasm::Func::make(store, fn_type_none_none.get(), bench_start, timer));
  available_functions[ImportName("bench", "end")] = std::move(
      wasm::Func::make(store, fn_type_none_none.get(), bench_end, timer));
  wasm::Func::callback fn = [](auto p, auto r) -> wasm::own<wasm::Trap> {
    return nullptr;
  };
  available_functions[ImportName("wasi_snapshot_preview1", "proc_exit")] =
      std::move(wasm::Func::make(store, fn_type_i32_none.get(), fn));
  available_functions[ImportName("wasi_snapshot_preview1", "fd_write")] =
      std::move(wasm::Func::make(store, fn_type_4xi32_i32.get(), fn));

  // Construct list of import functions.
  auto import_names = list_imports(module);
  std::vector<const wasm::Extern*> imports = {};
  for (ImportName i : import_names) {
    auto found = available_functions.find(i) != available_functions.end();
    if (found) {
      imports.push_back(available_functions[i].get());
    } else {
      std::cerr << "Unable to find a function for import: " << i.first << " "
                << i.second << std::endl;
      exit(1);
    }
  }

  return imports;
}
