project(AppR VERSION 1.0)

include(ExternalProject)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_RELEASE "")
    set(TARGET_DIR "debug")
else ()
    set(CARGO_RELEASE "--release")
    set(TARGET_DIR "release")
endif ()

set(CARGO_FLAGS ${CARGO_RELEASE}
        --manifest-path "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
        --target-dir "${CMAKE_CURRENT_BINARY_DIR}")

set(SRC_FILES src/main.rs Cargo.toml Cargo.lock build.rs)



set(LINK_PATHS "${CMAKE_CURRENT_BINARY_DIR}/../CoreC,${CMAKE_CURRENT_BINARY_DIR}/../WrapperC")
set(INCLUDE_PATHS "${CMAKE_SOURCE_DIR}/CoreC/include,${CMAKE_SOURCE_DIR}/WrapperC/include")

add_custom_target(app_r ALL
        DEPENDS ${SRC_FILES} wrapper_r wrapper_c core_r core_c)

add_custom_command(TARGET app_r
        COMMAND LINK_PATHS=${LINK_PATHS} INCLUDE_PATHS=${INCLUDE_PATHS} cargo build ${CARGO_FLAGS}
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/app_r)

add_dependencies(app_r core_c core_r wrapper_c wrapper_r)

#add_executable(app_r_bin IMPORTED GLOBAL)
#add_dependencies(app_r_bin app_r)

install(PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/app_r DESTINATION bin)
