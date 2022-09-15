project(CoreR VERSION 1.0)

include(ExternalProject)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_RELEASE "")
  set(TARGET_DIR "debug")
else ()
  set(CARGO_RELEASE "--release")
  set(TARGET_DIR "release")
endif ()

set(CARGO_FLAGS ${CARGO_RELEASE} --manifest-path "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml" --target-dir "${CMAKE_CURRENT_BINARY_DIR}")
ExternalProject_Add(core_r
#        BINARY_DIR ${TARGET_DIR}
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build ${CARGO_FLAGS}
        INSTALL_COMMAND ""
        TEST_COMMAND cargo test ${CARGO_FLAGS})
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libcore_r.a DESTINATION lib)