cmake_minimum_required(VERSION 3.27.6)

project(elfshaker VERSION "0.0.17")

enable_testing()

include(FetchContent)
set(FETCHCONTENT_QUIET OFF CACHE BOOL "" FORCE)

#FetchContent_Declare(
#    Corrosion
#    GIT_REPOSITORY https://github.com/tipi-build/corrosion.git
#    GIT_TAG c6132b645d1a0061a5c29ccf6a1d3c6f90490829 # v0.5 patched to support test execution, see https://github.com/tipi-build/corrosion/pull/1
#)
#FetchContent_MakeAvailable(Corrosion)
add_subdirectory(cmake/corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(elfshaker-cxxbridge CRATE elfshaker FILES lib.rs repo/repository.rs)
set(corrosion_generated_dir "${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated")
set(corrosion_generated_headers "${corrosion_generated_dir}/cxxbridge/elfshaker-cxxbridge/include")

FetchContent_Declare(
  Boost
  GIT_REPOSITORY https://github.com/boostorg/boost.git
  # boost 1.85
  # GIT_TAG         ab7968a0bbcf574a7859240d1d8443f58ed6f6cf
  # boost 1.80
  GIT_TAG         32da69a36f84c5255af8a994951918c258bac601
)
FetchContent_MakeAvailable(Boost)
find_package(boost_system CONFIG REQUIRED)
find_package(boost_filesystem CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)
find_package(boost_included_unit_test_framework CONFIG REQUIRED)


FetchContent_Declare(
  cpp-pre
  #SOURCE_DIR /Users/daminetreg/workspace/cpp-pre/file
  GIT_REPOSITORY https://github.com/cpp-pre/file.git
  GIT_TAG        57780f632473360d4daee3a868276498f946ffb6
)
FetchContent_MakeAvailable(cpp-pre)
find_package(cpp-pre_file CONFIG REQUIRED)


set(include_install_dir "include")

add_executable(test_cxxbridge tests/test_cxxbridge.cpp)
target_link_libraries(test_cxxbridge PUBLIC elfshaker-cxxbridge cpp-pre_file::file Boost::filesystem Boost::included_unit_test_framework)
if (CMAKE_SYSTEM_NAME MATCHES Darwin)
    target_link_libraries(test_cxxbridge PUBLIC "-framework Foundation")
endif()
add_test(NAME test_cxxbridge COMMAND $<TARGET_FILE:test_cxxbridge>) 



# Installing

# Layout. This works for all platforms:
#   * <prefix>/lib/cmake/<PROJECT-NAME>
#   * <prefix>/lib/
#   * <prefix>/include/
set(config_install_dir "lib/cmake/${PROJECT_NAME}")

set(generated_dir "${CMAKE_CURRENT_BINARY_DIR}/generated")

# Configuration
set(version_config "${generated_dir}/${PROJECT_NAME}ConfigVersion.cmake")
set(project_config "${generated_dir}/${PROJECT_NAME}Config.cmake")
set(targets_export_name "${PROJECT_NAME}Targets")
set(namespace "${PROJECT_NAME}::")

# Include module with fuction 'write_basic_package_version_file'
include(CMakePackageConfigHelpers)

# Configure '<PROJECT-NAME>ConfigVersion.cmake'
# Note: PROJECT_VERSION is used as a VERSION
write_basic_package_version_file(
    "${version_config}" COMPATIBILITY SameMajorVersion
)

# Configure '<PROJECT-NAME>Config.cmake'
# Use variables:
#   * targets_export_name
#   * PROJECT_NAME
configure_package_config_file(
    "cmake/modules/Config.cmake.in"
    "${project_config}"
    INSTALL_DESTINATION "${config_install_dir}"
)


# Targets:
install(
    TARGETS elfshaker-cxxbridge 
    EXPORT "${targets_export_name}"
    LIBRARY DESTINATION "lib"
    ARCHIVE DESTINATION "lib"
    RUNTIME DESTINATION "bin"
    INCLUDES DESTINATION "${include_install_dir}"
)

install(
  IMPORTED_RUNTIME_ARTIFACTS elfshaker_executable
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin"
)

# Headers:
install(
    DIRECTORY ${corrosion_generated_headers}/
    DESTINATION "${include_install_dir}"
    FILES_MATCHING PATTERN "*.[ih]*"
    PATTERN build/ EXCLUDE
    PATTERN "/build*" EXCLUDE 
    PATTERN ".git/*" EXCLUDE 
    PATTERN ".tipistore/*" EXCLUDE 
    PATTERN "doc/*" EXCLUDE 
    PATTERN "node_modules/*" EXCLUDE 
)

# Config
#   * <prefix>/lib/cmake/file/fileConfig.cmake
#   * <prefix>/lib/cmake/file/fileConfigVersion.cmake
#   * <prefix>/lib/cmake/file/fileTargets.cmake
install(
    FILES "${project_config}" "${version_config}"
    DESTINATION "${config_install_dir}"
)

install(
    EXPORT "${targets_export_name}"
    NAMESPACE "${namespace}"
    DESTINATION "${config_install_dir}"
)