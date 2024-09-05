cmake_minimum_required(VERSION 3.27.6)

project(elfshaker VERSION "0.0.16")

enable_testing()

include(FetchContent)
set(FETCHCONTENT_QUIET OFF CACHE BOOL "" FORCE)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/tipi-build/corrosion.git
    GIT_TAG c6132b645d1a0061a5c29ccf6a1d3c6f90490829 # v0.5 patched to support test execution, see https://github.com/tipi-build/corrosion/pull/1
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(elfshaker-cxxbridge CRATE elfshaker FILES lib.rs repo/repository.rs)

FetchContent_Declare(
  Boost
  GIT_REPOSITORY https://github.com/boostorg/boost.git
  # boost 1.85
   GIT_TAG         ab7968a0bbcf574a7859240d1d8443f58ed6f6cf
)
FetchContent_MakeAvailable(Boost)

FetchContent_Declare(
  cpp-pre
  #SOURCE_DIR /Users/daminetreg/workspace/cpp-pre/file
  GIT_REPOSITORY https://github.com/cpp-pre/file.git
  GIT_TAG        fda36ebd1d62ba846b6a03974293fbd87be4b433
)
FetchContent_MakeAvailable(cpp-pre) 


add_executable(test_cxxbridge tests/test_cxxbridge.cpp)
target_link_libraries(test_cxxbridge PUBLIC elfshaker-cxxbridge cpp-pre_file::file Boost::filesystem Boost::included_unit_test_framework)
if (CMAKE_SYSTEM_NAME MATCHES Darwin)
    target_link_libraries(test_cxxbridge PUBLIC "-framework Foundation")
endif()
add_test(NAME test_cxxbridge COMMAND $<TARGET_FILE:test_cxxbridge>) 