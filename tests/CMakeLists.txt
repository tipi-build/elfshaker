include(FetchContent)
set(FETCHCONTENT_QUIET OFF CACHE BOOL "" FORCE)

FetchContent_Declare(
  Boost
  GIT_REPOSITORY https://github.com/nxxm/boost.git
  # boost-1.89.0 with updates
  GIT_TAG        23090bb21676cb22f1d4ceed21d59fda8b420cc9
  EXCLUDE_FROM_ALL
)
FetchContent_MakeAvailable(Boost)
find_package(boost_system CONFIG)
find_package(boost_filesystem CONFIG)
find_package(boost_uuid CONFIG)
find_package(boost_included_unit_test_framework CONFIG)

FetchContent_Declare(
  cpp-pre
  GIT_REPOSITORY https://github.com/cpp-pre/file.git
  GIT_TAG        d6e2f875e70de332cb678bfcfbff32e8d51ad78d
)
FetchContent_MakeAvailable(cpp-pre)

find_package(cpp-pre_file CONFIG)

add_executable(test_cxxbridge test_cxxbridge.cpp)
target_link_libraries(test_cxxbridge PUBLIC
  elfshaker-cxxbridge
  cpp-pre_file::file
  Boost::filesystem
  Boost::included_unit_test_framework
)

if (CMAKE_SYSTEM_NAME MATCHES Darwin)
  target_link_libraries(test_cxxbridge PUBLIC "-framework Foundation")
endif()

add_test(NAME test_cxxbridge COMMAND $<TARGET_FILE:test_cxxbridge>)
