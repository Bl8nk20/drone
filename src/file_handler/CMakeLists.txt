set(file_handler_LIB_Name file_handler)

set(file_handler_LIB_Headers
    include/${file_handler_LIB_Name}/file_handler.hpp
)

set(file_handler_LIB_Sources
    file_handler.cpp
)

add_library(${file_handler_LIB_Name} 
    STATIC
    ${file_handler_LIB_Headers} 
    ${file_handler_LIB_Sources})

target_include_directories( ${file_handler_LIB_Name} 
    PUBLIC 
    include)
    
target_link_libraries(${file_handler_LIB_Name} 
    PRIVATE 
    nlohmann_json::nlohmann_json)