set(PID_FILTER_LIB_Name PID_Filter)

set(PID_FILTER_LIB_Header
    include/${PID_FILTER_LIB_Name}/PID_Filter.hpp
)

set(PID_FILTER_LIB_Source
    PID_Filter.cpp
)

add_library(${PID_FILTER_LIB_Name} 
    STATIC
    ${PID_FILTER_LIB_Header} 
    ${PID_FILTER_LIB_Source})

target_include_directories( ${PID_FILTER_LIB_Name} 
    PUBLIC 
    include)
    
target_link_libraries(${PID_FILTER_LIB_Name} 
    PRIVATE 
    FilterTemplate)