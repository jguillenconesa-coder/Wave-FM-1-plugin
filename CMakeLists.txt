cmake_minimum_required(VERSION 3.24)
project(WaveFM1Windows VERSION 0.1.0 LANGUAGES C CXX)
if(NOT WIN32 OR NOT MSVC OR NOT CMAKE_SIZEOF_VOID_P EQUAL 8)
    message(FATAL_ERROR "This packaging project requires 64-bit Windows and MSVC.")
endif()
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_TLS_VERIFY TRUE CACHE BOOL "Verify TLS certificates for dependency downloads" FORCE)
add_compile_options(/guard:cf)
add_link_options(/DYNAMICBASE /NXCOMPAT /HIGHENTROPYVA /guard:cf)
set(CLAP_WRAPPER_DOWNLOAD_DEPENDENCIES TRUE CACHE BOOL "" FORCE)
include(FetchContent)
FetchContent_Declare(clap-wrapper
    GIT_REPOSITORY https://github.com/free-audio/clap-wrapper.git
    GIT_TAG 1cca996e96f29ab2be7ae9f8cfe532bbc92e1dd6)
FetchContent_MakeAvailable(clap-wrapper)
add_library(wave_fm1 MODULE)
# Keep the upstream name: clap-wrapper searches standard CLAP directories for
# a module with the same basename as its VST3 binary.
target_add_vst3_wrapper(TARGET wave_fm1 OUTPUT_NAME "FM-1"
    BUNDLE_VERSION "${PROJECT_VERSION}" WINDOWS_FOLDER_VST3 FALSE)
install(TARGETS wave_fm1 RUNTIME DESTINATION VST3 LIBRARY DESTINATION VST3)
