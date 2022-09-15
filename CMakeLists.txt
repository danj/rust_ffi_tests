cmake_minimum_required(VERSION 3.23)
project(ffi_tests VERSION 1.0)

add_subdirectory(CoreC)
add_subdirectory(CoreR)

add_subdirectory(WrapperC)
add_subdirectory(WrapperR)

add_subdirectory(AppC)

