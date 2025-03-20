set(file_writer_LIB_Name Write_Files)

set(file_writer_LIB_Headers
    include/${file_writer_LIB_Name}/Write_Files.hpp
)

set(file_writer_LIB_Sources
    Write_Files.cpp
)

add_library(${file_writer_LIB_Name}
        STATIC
        ${file_writer_LIB_Headers}
        ${file_writer_LIB_Sources})

target_include_directories(${file_writer_LIB_Name} INTERFACE include)

target_link_libraries(${file_writer_LIB_Name} PRIVATE
    nlohmann_json::nlohmann_json)