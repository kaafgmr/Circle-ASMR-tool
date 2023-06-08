# Install script for directory: C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/raylib/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/build/raylib/raylib.lib")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/raylib/src/raylib.h"
    "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/raylib/src/rlgl.h"
    "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/raylib/src/raymath.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/build/raylib/raylib.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/build/raylib/raylib-config-version.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/raylib/src/../cmake/raylib-config.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("C:/Users/Kelson/Desktop/RustLearing/circle_asmr_thing/target/debug/build/raylib-sys-360ec88f2d54187b/out/build/raylib/external/glfw/cmake_install.cmake")

endif()

