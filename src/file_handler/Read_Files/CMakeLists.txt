set(file_reader_LIB_Name Read_Files)

set(file_reader_LIB_Headers
    include/${file_reader_LIB_Name}/Read_Files.hpp
)

set(file_reader_LIB_Sources
    Read_Files.cpp
)

add_library(${file_reader_LIB_Name} 
    STATIC
    ${file_reader_LIB_Headers} 
    ${file_reader_LIB_Sources})

target_include_directories( ${file_reader_LIB_Name} 
    PUBLIC 
    include)
    
target_link_libraries(${file_reader_LIB_Name} 
    PRIVATE 
    nlohmann_json::nlohmann_json)