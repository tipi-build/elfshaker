cmake_minimum_required(VERSION 3.27.6)

project(elfshaker VERSION "0.0.17")

if (BUILD_TESTING)
  enable_testing()
endif()

set(CORROSION_VERBOSE_OUTPUT ON)
add_subdirectory(cmake/corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(elfshaker-cxxbridge CRATE elfshaker FILES lib.rs repo/repository.rs)
set(corrosion_generated_dir "${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated")
set(corrosion_generated_headers "${corrosion_generated_dir}/cxxbridge/elfshaker-cxxbridge/include")

set(include_install_dir "include")

if (BUILD_TESTING)
  add_subdirectory(tests)
endif()

if(CMAKE_SYSTEM_NAME MATCHES "Linux")
  target_link_libraries(elfshaker INTERFACE ${CMAKE_DL_LIBS})
  target_link_libraries(elfshaker-cxxbridge INTERFACE ${CMAKE_DL_LIBS})
endif()

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

corrosion_install(
    TARGETS elfshaker
    EXPORT "${targets_export_name}"
    LIBRARY DESTINATION "lib"
    ARCHIVE DESTINATION "lib"
    RUNTIME DESTINATION "bin"
    INCLUDES DESTINATION "${include_install_dir}")

# Install all generated files
install(FILES
  ${CMAKE_CURRENT_BINARY_DIR}/corrosion/${targets_export_name}Corrosion.cmake
  DESTINATION ${config_install_dir}
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
 
