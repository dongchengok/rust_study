cmake_minimum_required(VERSION 3.0.0)
project(call_rust VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(call_rust main.cpp)
target_link_directories(call_rust PRIVATE lib)
target_link_libraries(call_rust PRIVATE c_call_rust)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
