add_executable(${PROJECT_NAME})

file(GLOB_RECURSE SOURCES *.cc *.h *.hh)

target_sources(${PROJECT_NAME} PRIVATE ${SOURCES})

target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src)

# definitions for Logger
target_compile_definitions(${PROJECT_NAME}
  PUBLIC
    $<$<CONFIG:Debug>:YARUSTO_DEBUG>
    $<$<CONFIG:Release>:YARUSTO_RELEASE>
)

target_link_libraries(${PROJECT_NAME}
  PRIVATE
    Boost::boost
    Boost::system
    version::version
)