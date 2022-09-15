cmake_minimum_required(VERSION 3.23)
project(WrapperC VERSION 1.0)

add_library(wrapper_c STATIC)
target_sources(wrapper_c
        PRIVATE src/wrapper_c.cpp
        PUBLIC FILE_SET HEADERS
        BASE_DIRS include
        FILES include/wrapper_c.h)

install(TARGETS wrapper_c FILE_SET HEADERS)

target_link_libraries(wrapper_c core_c core_r)

