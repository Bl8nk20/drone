#Patterns:
# Template Method – generische Filterlogik, konkrete Implementierung EKF, KF, LPF
# Decorator – Filterketten (optional)

add_subdirectory(ekf)
add_subdirectory(kf)
add_subdirectory(lpf)
add_subdirectory(pid_filter)
add_subdirectory(kalmann_filter)
