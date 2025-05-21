# The Flutter tooling requires that developers have a version of Visual Studio
# installed that includes CMake 3.14 or later. You should not increase this
# version, as doing so will cause the plugin to fail to compile for some
# customers of the plugin.
cmake_minimum_required(VERSION 3.14)

# Project-level configuration.
set(PROJECT_NAME "image_combine")
project(${PROJECT_NAME} LANGUAGES CXX)

include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust image_combine "") # This provides ${image_combine_cargokit_lib}

# Define the plugin library, including the C++ shim.
# This library should be named after the plugin, e.g., "image_combine.dll".
add_library(${PROJECT_NAME} SHARED
  "image_combine_plugin.cpp"
)

# Link the plugin library (C++) against the Rust library.
target_link_libraries(${PROJECT_NAME} PRIVATE ${${PROJECT_NAME}_cargokit_lib})

# Set the bundled libraries variable to the library we just defined.
# This tells Flutter to package "image_combine.dll".
set(image_combine_bundled_libraries
  "$<TARGET_FILE:${PROJECT_NAME}>"
  PARENT_SCOPE
)
