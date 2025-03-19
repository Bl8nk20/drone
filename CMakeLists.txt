cmake_minimum_required(VERSION 3.22)
project(Mini_UAV 
        VERSION 1.0 
        LANGUAGES C CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)

# set library outputs
set(LIBRARY_OUTPUT_PATH ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})

# user updating while running
message("CXX Standard: ${CMAKE_CXX_STANDARD}")
message("CMAKE_CURRENT_BINARY_DIR   " ${CMAKE_CURRENT_BINARY_DIR}/build/)
message("CMAKE_SOURCE_DIR:          " ${CMAKE_SOURCE_DIR})

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    # adding options like testing and so
    option(ENABLE_TESTING "Enable a Unit Testing Build" OFF)


    # set cmake files
    set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake/")
    include(AddGitSubmodule)
    include(GitFetchRepos)
    include(Docs)

endif()

add_subdirectory(src)