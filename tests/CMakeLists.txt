include(Catch)

set(Test_NAME "UnitTests")
set(Test_Libraries
    UAV::data_types)

set(Test_Sources
    "data_types.cpp")

add_executable(${Test_NAME} ${Test_Sources})

target_link_libraries(${Test_NAME} PUBLIC
    Catch2::Catch2WithMain
    ${Test_Libraries})

catch_discover_tests(${Test_NAME})