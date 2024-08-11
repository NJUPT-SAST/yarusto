cmake_minimum_required(VERSION 3.21)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_LIST_DIR}/.cmake/)
include(Version)

if(APPLE)
  set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64" CACHE STRING "" FORCE)
endif()

if (POLICY CMP0167)
  cmake_policy(SET CMP0167 OLD)
endif()

project(yarusto VERSION ${VERSION_SEMANTIC} LANGUAGES CXX)

# find vcpkg-installed
find_package(Boost REQUIRED COMPONENTS system)

# source code
add_subdirectory(src)
# cmd arg parser
add_subdirectory(3rdpart/argparse)
# reflect-cpp
add_subdirectory(3rdpart/reflect-cpp)

# install
install(TARGETS ${PROJECT_NAME}
  BUNDLE DESTINATION .
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)