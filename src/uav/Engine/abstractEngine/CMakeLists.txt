# abstract Engine:
# DP: abstract representation – gemeinsame Schnittstelle für verschiedene Engine-Typen

set(ABSTRACT_ENGINE_TEMPLATE abstractEngine)
set(LIB_HEADERS include/${ABSTRACT_ENGINE_TEMPLATE}/AabstractEngine.hpp)

add_library(${ABSTRACT_ENGINE_TEMPLATE}
    INTERFACE
    ${LIB_HEADERS})

target_include_directories(${ABSTRACT_ENGINE_TEMPLATE} INTERFACE include)