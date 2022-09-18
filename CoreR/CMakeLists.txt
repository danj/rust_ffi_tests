project(CoreR VERSION 1.0)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_RELEASE "")
  set(TARGET_DIR "debug")
else ()
  set(CARGO_RELEASE "--release")
  set(TARGET_DIR "release")
endif ()

set(CARGO_FLAGS ${CARGO_RELEASE} --manifest-path "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml" --target-dir "${CMAKE_CURRENT_BINARY_DIR}")
set(SRC_FILES src/lib.rs Cargo.toml Cargo.lock build.rs)

add_custom_target(libcore_r
  DEPENDS ${SRC_FILES})

add_custom_command(TARGET libcore_r
  COMMAND cargo build ${CARGO_FLAGS})

add_library(core_r STATIC IMPORTED GLOBAL)
add_dependencies(core_r libcore_r)

set_target_properties(core_r PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libcore_r.a)
target_include_directories(core_r INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/ffi)
target_link_libraries(core_r INTERFACE dl pthread m)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libcore_r.a DESTINATION lib)
install(FILES ${CMAKE_CURRENT_SOURCE_DIR}/ffi/core_r.h DESTINATION include)

add_test(NAME CoreR
  COMMAND cargo test ${CARGO_FLAGS})