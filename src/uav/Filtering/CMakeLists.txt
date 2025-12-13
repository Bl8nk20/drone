#Patterns:
# Template Method – generische Filterlogik, konkrete Implementierung EKF, KF, LPF
# Decorator – Filterketten (optional)

add_subdirectory(abstractFilter)
add_subdirectory(ekf)
add_subdirectory(lpf)
add_subdirectory(kalmann_filter)
add_subdirectory(filterfactory)