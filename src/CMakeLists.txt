add_executable(${PROJECT_NAME})

file(GLOB_RECURSE SOURCES *.cpp *.h *.hpp)

target_sources(${PROJECT_NAME} PRIVATE ${SOURCES})

target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src)

target_compile_definitions(${PROJECT_NAME}
  PUBLIC
    $<$<CONFIG:Debug>:YARUSTO_DEBUG>
    $<$<CONFIG:Release>:YARUSTO_RELEASE>
)

set(REFLECTCPP_TOML ON) # Optional
set(REFLECTCPP_YAML ON) # Optional

target_link_libraries(${PROJECT_NAME}
  PRIVATE
    Boost::boost
    Boost::system
    argparse
    reflectcpp
    version::version
)