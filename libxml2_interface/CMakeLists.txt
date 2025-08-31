cmake_minimum_required(VERSION 3.22)
project(libxml2_interface C)
set(CMAKE_C_STANDARD 11)

add_library(libxml2_interface INTERFACE)
target_include_directories(
        libxml2_interface
        INTERFACE
        $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

target_sources(
        libxml2_interface
        INTERFACE wrapper.h
)

find_package(LibXml2 REQUIRED)

target_link_libraries(libxml2_interface INTERFACE LibXml2::LibXml2)
target_include_directories(libxml2_interface INTERFACE ${LIBXML2_INCLUDE_DIR} ${LIBXML2_INCLUDE_DIR}/libxml)

install(
        TARGETS libxml2_interface
        PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_PREFIX}/include
)

add_executable(TestInterface test_interface.c)
set_target_properties(TestInterface PROPERTIES LINKER_LANGUAGE C)
target_link_libraries(TestInterface libxml2_interface)
