cmake_minimum_required(VERSION 3.27)
project(WaveFM1Windows VERSION 0.2.0 LANGUAGES C CXX)
if(APPLE)
    enable_language(OBJCXX)
endif()
if(NOT CMAKE_SIZEOF_VOID_P EQUAL 8 OR (NOT APPLE AND (NOT WIN32 OR (NOT MSVC AND NOT MINGW))))
    message(FATAL_ERROR "This packaging project requires Windows x64 or macOS 64-bit.")
endif()
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_TLS_VERIFY TRUE CACHE BOOL "Verify TLS certificates for dependency downloads" FORCE)
if(MSVC)
    # All SDK/wrapper objects must use /MT like the static Rust companion.
    set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
    add_compile_options(/guard:cf)
    add_link_options(/DYNAMICBASE /NXCOMPAT /HIGHENTROPYVA /guard:cf)
elseif(MINGW)
    add_compile_options(-fstack-protector-strong)
    add_link_options(-static -static-libgcc -static-libstdc++ -Wl,--dynamicbase,--nxcompat,--high-entropy-va)
endif()
set(CLAP_WRAPPER_DOWNLOAD_DEPENDENCIES TRUE CACHE BOOL "" FORCE)
include(FetchContent)
find_package(Python3 REQUIRED COMPONENTS Interpreter)
FetchContent_Declare(clap-wrapper
    GIT_REPOSITORY https://github.com/free-audio/clap-wrapper.git
    GIT_TAG 1cca996e96f29ab2be7ae9f8cfe532bbc92e1dd6
    PATCH_COMMAND "${Python3_EXECUTABLE}" "${CMAKE_CURRENT_SOURCE_DIR}/tools/private_companion.py" "<SOURCE_DIR>")
FetchContent_MakeAvailable(clap-wrapper)
add_library(wave_fm1 MODULE)
# Keep matching binary basenames. The pin-checked patch loads the private
# companion beside this VST3, never a module from global CLAP directories.
set(VST3_TARGET wave_fm1) # pinned wrapper's macOS output-directory helper uses this name
target_add_vst3_wrapper(TARGET wave_fm1 OUTPUT_NAME "FM-1"
    SINGLE_PLUGIN_TUID "763245F44C814116B23B2268F75ADC65"
    BUNDLE_VERSION "${PROJECT_VERSION}" WINDOWS_FOLDER_VST3 TRUE
    ASSET_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bundle")
if(APPLE)
    set_target_properties(wave_fm1 PROPERTIES LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bundle")
endif()
set_target_properties(wave_fm1 PROPERTIES PREFIX "" OUTPUT_NAME "FM-1")
if(MINGW)
    target_link_libraries(wave_fm1 PRIVATE ssp)
endif()
install(DIRECTORY "${CMAKE_BINARY_DIR}/bundle/FM-1.vst3" DESTINATION VST3)
add_executable(wave_fm1_probe EXCLUDE_FROM_ALL tools/probe-plugin.cpp)
if(MINGW)
    target_link_options(wave_fm1_probe PRIVATE -municode)
    target_link_libraries(wave_fm1_probe PRIVATE ssp)
endif()
