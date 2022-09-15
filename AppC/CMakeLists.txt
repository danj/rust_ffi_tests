cmake_minimum_required(VERSION 3.23)
project(AppC VERSION 1.0)

add_executable(app_c src/main.cpp)
target_link_libraries(app_c core_c)

install(TARGETS app_c)


