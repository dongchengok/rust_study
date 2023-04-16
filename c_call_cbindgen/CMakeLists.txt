cmake_minimum_required(VERSION 3.0.0)
project(call_cbindgen VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(call_cbindgen src/main.cpp)
target_link_directories(call_cbindgen PRIVATE lib)
target_link_libraries(call_cbindgen PRIVATE c_call_cbindgen)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
