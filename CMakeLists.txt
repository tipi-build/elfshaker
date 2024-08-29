cmake_minimum_required(VERSION 3.27.6)

project(elfshaker VERSION "0.0.16")

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(elfshaker-cxxbridge CRATE elfshaker FILES lib.rs repo/repository.rs)

add_executable(your_cpp_bin main.cpp)
target_link_libraries(your_cpp_bin PUBLIC elfshaker-cxxbridge)
if (CMAKE_SYSTEM_NAME MATCHES Darwin)
    target_link_libraries(your_cpp_bin PUBLIC "-framework Foundation")
endif()
 