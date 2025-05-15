cmake_minimum_required(VERSION 3.31)
project(webhooks_c C)

set(CMAKE_C_STANDARD 23)

add_executable(webhooks_c main.c cJSON/cJSON.c cJSON/cJSON.h)

target_link_libraries(webhooks_c curl)
target_link_libraries(webhooks_c discord)
