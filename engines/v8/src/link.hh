#include <string>
#include <utility>
#include <vector>
#include "bench-state.hh"
#include "wasm.hh"

using ImportName = std::pair<std::string, std::string>;
std::vector<ImportName> list_imports(const wasm::Module* module);
std::vector<wasm::own<wasm::Func>> link(wasm::Store* store,
                                        const wasm::Module* module,
                                        Timer* timer);
wasm::own<wasm::Extern> find_start_fn(const wasm::Module* module,
                                      const wasm::Instance* instance);
