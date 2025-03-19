set(Lib_NAME FilterTemplate)

set(LIB_Headers
    include/${Lib_NAME}/IFilterTemplate.hpp
)

add_library(${Lib_NAME}
        INTERFACE
        ${LIB_Headers}
)

target_include_directories(${Lib_NAME} INTERFACE include)