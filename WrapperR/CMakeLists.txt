project(WrapperR VERSION 1.0)

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
ExternalProject_Add(cargo_wrapper_r
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND EXTRA_LINK_PATH=${CMAKE_CURRENT_BINARY_DIR}/../CoreC cargo build ${CARGO_FLAGS}
        INSTALL_COMMAND ""
        TEST_COMMAND cargo test ${CARGO_FLAGS}
        DEPENDS core_c core_r)

add_library(wrapper_r STATIC IMPORTED GLOBAL)
set_target_properties(wrapper_r PROPERTIES
    IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libwrapper_r.a)
target_include_directories(wrapper_r INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/ffi)
target_link_libraries(wrapper_r INTERFACE dl pthread m)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libwrapper_r.a DESTINATION lib)
install(FILES ${CMAKE_CURRENT_SOURCE_DIR}/ffi/wrapper_r.h DESTINATION include)
