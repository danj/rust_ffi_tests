cmake_minimum_required(VERSION 3.23)
project(CoreC VERSION 1.0)

add_library(core_c STATIC)
target_sources(core_c
    PRIVATE src/core_c.cpp
    PUBLIC FILE_SET HEADERS
    BASE_DIRS include
    FILES include/core_c.h)

install(TARGETS core_c FILE_SET HEADERS)


