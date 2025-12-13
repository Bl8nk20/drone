#Patterns:
# Strategy Pattern – verschiedene Transportlayer (UDP, Serial, Mock)
# Factory Method / Abstract Factory – Parser + Message-Builder
# PIMPL – stabile API, interne Implementierung austauschbar

add_subdirectory(messagebuilder)
add_subdirectory(sender)
add_subdirectory(receiver)

add_subdirectory(commHandler)
