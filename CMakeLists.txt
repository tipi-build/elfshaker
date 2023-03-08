cmake_minimum_required(VERSION 3.17.0)
project(elfshaker VERSION "0.0.1")
enable_testing()

add_subdirectory(contrib/corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)
