# 🚁 Drone Project (C++20, Modular Architecture, UAV + Ground Station)

Dieses Repository enthält die vollständige Software-Architektur für eine **sub-250g Drohne**, entwickelt in **Modern C++20**, mit einer klar modularisierten Struktur für:

- **UAV (Flight Controller)**  
- **Ground Station (Laptop/Controller)**  
- **Shared Libraries für Kommunikation, Koordinaten, Enums, Protokoll etc.**

Das langfristige Ziel ist eine **skalierbare, erweiterbare und hardwareunabhängige** Drone-Software, die sowohl kleine als auch größere UAVs unterstützen kann.

---

## ✨ Features (Current & Planned)

### ✔ UAV (Flight Controller)

- Modulare Architektur (Sensors, Filtering, Controls, Engine/Mixing)
- Hardware-Abstraction-Layer (HAL) für zukünftige Sensoren/Boards
- Sensorfusion (IMU, Barometer, GPS) – *Complementary Filter → später EKF*
- Mehrere Flight Modes (Manual, Stabilize, AltHold, PositionHold – TODO)
- Motor Mixing & Actuator Control
- Failsafe-Mechanismen (TODO)
- Telemetrie & Logging (TODO)

### ✔ Ground Station

- UI Layer (Laptop / Custom Controller)
- Joystick → Command Converter
- Live Telemetry & UAV State
- Parameter Tooling (TODO)
- Map/Waypoint GUI (optional)

### ✔ Shared Libraries

- Kommunikationsprotokoll (ähnlich zu MAVLink, aber schlanker)
- Message Definitions
- Coordinates/Motion Structures
- Common Enums
- Utility & Math Modules

---

## 📁 Projektstruktur

```
DRONE
│
├── app/                 # Executables für UAV oder Ground Station
├── build/               # (Ignored) Build output
├── cmake/               # CMake scripts, toolchains, helper modules
├── config/              # UAV config files (PID, parameters)
├── configured/          # Auto-generated files
├── doc/                 # Documentation, diagrams, notes
├── external/            # Third-party libs (optional)
│
├── src/
│   ├── ground_station/
│   │   ├── Controls/
│   │   ├── Convert_Inputs/
│   │   ├── Services/
│   │   ├── UI/
│   │   └── CMakeLists.txt
│   │
│   ├── shared/
│   │   ├── communication/
│   │   ├── Coordinates/
│   │   ├── enum/
│   │   └── CMakeLists.txt
│   │
│   ├── uav/
│   │   ├── Controls/
│   │   ├── Engine/
│   │   ├── Filtering/
│   │   ├── Sensor/
│   │   └── CMakeLists.txt
│   │
│   └── CMakeLists.txt
│
├── tests/               # GoogleTest unit tests
│
├── .gitmodules
├── .gitignore
├── CMakeLists.txt       # Root build configuration
├── Makefile             # Call CMake more easily (optional)
└── README.md
```
---

## 🔧 Build Instructions

### Prerequisites

- CMake ≥ 3.25
- GCC ≥ 11 / Clang ≥ 13 with C++20 support
- git, make, ninja (optional)
- On UAV:  
  - Raspberry Pi OS Lite (32-bit) / Banana Pi Zero OS
  - WiringPi / SPI / I2C libraries (optional depending on HAL)

### Clone

```bash
git clone https://github.com/Bl8nk20/drone
cd drone
```

### Build (Default)

```bash
make prepare
make install
make compile
```

### Build Tests

```bash
make prepare
make install
make testing
```

---

## 🧱 Architektur

Die Software folgt einer **klaren Schichtenarchitektur**, inspiriert von PX4 und ArduPilot – jedoch in modernem C++20 umgesetzt:

```
Sensors → Filtering → State Estimation → Controls → Engine/Mixing → Motors
                          ↑                ↓
                    Telemetry/Comm ← Ground Station
```

### 🔹 UAV Layer

- **Sensors:** IMU, Barometer, Magnetometer, GPS (via HAL)
- **Filtering:** Low-pass Filtering, Sensor Fusion
- **State Estimation:** Orientation, Speed, Altitude
- **Controls:** PID-Loops, Flight Modes, Motor Mixer
- **Engine:** ESC Signale / PWM Output

### 🔹 Ground Station Layer

- Visualisierung
- Setup/Parameter
- Joystick → Control Signals
- Telemetrie

### 🔹 Shared Libraries

- Netzwerk & Serial Communication
- Messages & IDs
- Coordinate Math (ENU, NED, Geo → Local)

---

## 🛰️ Kommunikation (UAV ↔ Ground Station)

Wird über ein **leichtgewichtiges Binary-Protokoll** realisiert.

### Eigenschaften

- Fixe Header + CRC
- Variable Payload
- Message IDs mit Versionsnummern
- Unterstützung für UDP oder Serial
- Optional: MAVLink-Kompatibilität

---

## Tests

Im Ordner `tests/` befinden sich Unit Tests (GoogleTest):

- Kommunikation: Parser, Message Builder
- Filter: Low-pass Filter, Complementary Filter
- Math Utilities
- HAL Mocks
- Controller (PID, Mixer)

Tests werden via CTest ausgeführt:

```bash
make testing
```

---

## 🗺 Roadmap

### ✔ Phase 1 (Current)

- Projektstruktur
- Shared Libraries
- Basic UAV Loop
- Sensor Interfaces
- Basic Filters

### 🔜 Phase 2

- Full Control Pipeline
- Telemetry
- Joystick Control
- Stabilize Mode

### 🔮 Phase 3

- Altitude Hold
- Position Hold
- Mission Mode
- Mapping / Visualization
- Autonomous Flight

---

## 🤝 Contribution

Pull Requests sind willkommen!
Bitte halte dich an die C++20 Coding Guidelines im `doc/` Ordner.

---

## 📜 License

MIT License.
Details siehe `LICENSE` im Repo.

---

## 🙋 Support

Bei Fragen: GitHub Issues öffnen oder mich direkt kontaktieren.

---
