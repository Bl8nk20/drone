include(Catch)

set(Test_NAME "UnitTests")
set(Test_Libraries
    data_types
    PID_Filter)

set(Test_Sources
    data_types/data_types.cpp
    filter/pid_controller.cpp)

add_executable(${Test_NAME} PRIVATE ${Test_Sources})

target_link_libraries(${Test_NAME} PUBLIC
    ${Test_Libraries} 
    Catch2::Catch2WithMain)