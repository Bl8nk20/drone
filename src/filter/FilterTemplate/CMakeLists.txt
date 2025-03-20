set(FILTER_Lib_NAME FilterTemplate)

set(FILTER_LIB_Headers
    include/${FILTER_Lib_NAME}/IFilterTemplate.hpp
)

add_library(${FILTER_Lib_NAME}
        INTERFACE
        ${FILTER_LIB_Headers}
)

target_include_directories(${FILTER_Lib_NAME} INTERFACE include)