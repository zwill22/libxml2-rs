cmake_minimum_required(VERSION 4.0)
project(libxml2_interface C)
set(CMAKE_C_STANDARD 11)

add_library(libxml2_interface INTERFACE)
target_include_directories(libxml2_interface INTERFACE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>)
set_target_properties(libxml2_interface PROPERTIES LINKER_LANGUAGE C)


find_package(LibXml2 REQUIRED)

target_link_libraries(libxml2_interface INTERFACE LibXml2::LibXml2)

add_executable(TestInterface test_interface.c)
set_target_properties(TestInterface PROPERTIES LINKER_LANGUAGE C)
target_link_libraries(TestInterface libxml2_interface)
