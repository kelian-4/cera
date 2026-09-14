cmake_minimum_required(VERSION 3.16)
project(cera CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

add_compile_options(-Wall -Wextra)

option(CERA_WERROR "Traiter les warnings comme des erreurs" OFF)
if(CERA_WERROR)
    add_compile_options(-Werror)
endif()

add_library(cera_core
    src/terminal/Terminal.cpp
    src/model/Row.cpp
    src/model/Buffer.cpp
    src/syntax/Syntax.cpp
    src/syntax/CSyntax.cpp
    src/io/FileIO.cpp
    src/view/Colors.cpp
    src/view/ScreenRenderer.cpp
    src/editor/Editor.cpp
)
target_include_directories(cera_core PUBLIC src)

add_executable(cera src/main.cpp)
target_link_libraries(cera PRIVATE cera_core)

enable_testing()
add_executable(model_tests tests/model_tests.cpp)
target_link_libraries(model_tests PRIVATE cera_core)
add_test(NAME model_tests COMMAND model_tests)
