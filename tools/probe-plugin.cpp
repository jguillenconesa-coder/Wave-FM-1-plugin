// A minimal Windows ABI/load probe; no firmware, UI or DAW preferences are opened.
#include <windows.h>
#include <cstdint>
#include <iostream>

int wmain(int argc, wchar_t** argv) {
    if (argc != 2) { std::cerr << "Usage: probe-plugin <inner VST3 binary>\n"; return 2; }
    HMODULE module = LoadLibraryW(argv[1]);
    if (!module) { std::cerr << "LoadLibrary failed: " << GetLastError() << '\n'; return 1; }
    using ModuleFn = bool (*)();
    using FactoryFn = void* (*)();
    auto init = reinterpret_cast<ModuleFn>(GetProcAddress(module, "InitDll"));
    auto exit = reinterpret_cast<ModuleFn>(GetProcAddress(module, "ExitDll"));
    auto getFactory = reinterpret_cast<FactoryFn>(GetProcAddress(module, "GetPluginFactory"));
    if (!init || !exit || !getFactory || !init()) { std::cerr << "Module initialization failed\n"; return 1; }
    void* factory = getFactory();
    if (!factory) { std::cerr << "Private companion not found or factory initialization failed\n"; exit(); return 1; }
    void** table = *reinterpret_cast<void***>(factory);
    using CountFn = std::int32_t (*)(void*);
    using ReleaseFn = std::uint32_t (*)(void*);
    auto count = reinterpret_cast<CountFn>(table[4])(factory);
    reinterpret_cast<ReleaseFn>(table[2])(factory);
    const bool stopped = exit();
    FreeLibrary(module);
    if (count < 1 || !stopped) { std::cerr << "Invalid factory or shutdown\n"; return 1; }
    std::cout << "Private VST3 factory loaded: " << count << " classes; clean shutdown\n";
    return 0;
}
