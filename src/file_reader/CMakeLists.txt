set(file_reader_LIB_Name file_reader)

set(file_reader_LIB_Headers
    include/${file_reader_LIB_Name}/file_reader.hpp
)

set(file_reader_LIB_Sources
    File_Reader.cpp
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