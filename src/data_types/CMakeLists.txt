set(data_types_LIB_Name data_types)

set(data_types_LIB_Headers
    include/${data_types_LIB_Name}/GPS_Coordinates.hpp
)

add_library(${data_types_LIB_Name}
        INTERFACE
        ${data_types_LIB_Headers}
)

target_include_directories(${data_types_LIB_Name} INTERFACE include)