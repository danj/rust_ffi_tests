project(WrapperR VERSION 1.0)

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

set(SRC_FILES src/lib.rs Cargo.toml Cargo.lock build.rs)

add_custom_target(libwrapper_r
  DEPENDS ${SRC_FILES})

set(LINK_PATHS "${CMAKE_CURRENT_BINARY_DIR}/../CoreC")
set(INCLUDE_PATHS "${CMAKE_SOURCE_DIR}/CoreC/include")

add_custom_command(TARGET libwrapper_r
  COMMAND LINK_PATHS=${LINK_PATHS} INCLUDE_PATHS=${INCLUDE_PATHS} cargo build ${CARGO_FLAGS})

add_library(wrapper_r STATIC IMPORTED GLOBAL)
add_dependencies(wrapper_r libwrapper_r)

set_target_properties(wrapper_r PROPERTIES
    IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libwrapper_r.a)
target_include_directories(wrapper_r INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/ffi)
target_link_libraries(wrapper_r INTERFACE core_r core_c dl pthread m)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libwrapper_r.a DESTINATION lib)
install(FILES ${CMAKE_CURRENT_SOURCE_DIR}/ffi/wrapper_r.h DESTINATION include)

add_test(NAME WrapperR
        COMMAND cargo test ${CARGO_FLAGS})