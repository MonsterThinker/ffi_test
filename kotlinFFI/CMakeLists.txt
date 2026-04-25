cmake_minimum_required(VERSION 3.22.1)
project(HelloApp)

add_library(native-lib SHARED native-lib.c)

add_library(rustlib STATIC IMPORTED)

set_target_properties(rustlib PROPERTIES
    IMPORTED_LOCATION
    ${CMAKE_SOURCE_DIR}/librustlib.a
)

find_library(log-lib log)

target_link_libraries(
    native-lib
    rustlib
    ${log-lib}
)