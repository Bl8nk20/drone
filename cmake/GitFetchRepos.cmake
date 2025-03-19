include(FetchContent)

# Boost for more accuracy and bigger floats
#FetchContent_Declare(
#  multiprecision
#  GIT_REPOSITORY https://github.com/boostorg/multiprecision.git
#  GIT_TAG      Boost_1_86_0
#  GIT_SHALLOW TRUE
#)
#FetchContent_MakeAvailable(multiprecision)

if(ENABLE_TESTING)
    # Testing
    FetchContent_Declare(
        Catch2
        GIT_REPOSITORY https://github.com/catchorg/Catch2
        GIT_TAG v3.6.0B
        GIT_SHALLOW TRUE)
    FETCHCONTENT_MakeAvailable(Catch2)
    list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
endif()