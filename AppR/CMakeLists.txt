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

ExternalProject_Add(cargo_app_r
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND LINK_PATHS=${CMAKE_CURRENT_BINARY_DIR}/../CoreC,${CMAKE_CURRENT_BINARY_DIR}/../WrapperC INCLUDE_PATHS=${CMAKE_SOURCE_DIR}/CoreC/include,${CMAKE_SOURCE_DIR}/WrapperC/include cargo build ${CARGO_FLAGS}
        INSTALL_COMMAND ""
        TEST_COMMAND cargo test ${CARGO_FLAGS}
        DEPENDS wrapper_r wrapper_c)

add_executable(app_r IMPORTED GLOBAL)
install(PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/app_r DESTINATION bin)
