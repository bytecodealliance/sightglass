#include <string>
#include <utility>
#include <vector>
#include "bench-state.hh"
#include "wasm.hh"

using ImportName = std::pair<std::string, std::string>;
std::vector<ImportName> list_imports(const wasm::Module* module);
std::vector<const wasm::Extern*> link(wasm::Store* store,
                                      const wasm::Module* module,
                                      Timer* timer);
