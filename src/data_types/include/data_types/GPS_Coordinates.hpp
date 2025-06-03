#ifndef UAV_SOFTWARE_IDATA_GPS_H
#define UAV_SOFTWARE_IDATA_GPS_H

#include <array>
#include <cmath>
#include <string>
#include <sstream>
#include <iomanip>

namespace UAV::Software::Data_Types{
    
    /**
     * @brief 
     * 
     */
    class GPS_Coordinates{
        private:
        static constexpr double kEarthRadius_m = 6371000.0;
        const std::array<double,3> m_lla{};       // {lat, lon, alt}
        const std::chrono::milliseconds m_timestamp{}; // Unix‐ms

        bool isValid() const;
        float distanceTo(const GPS_Coordinates& other) const;


        public:
        GPS_Coordinates() = default;
        GPS_Coordinates(double lat_deg,
                        double lon_deg,
                        double alt_m = 0.0,
                        std::chrono::milliseconds ts = std::chrono::milliseconds{0})
            : m_lla{lat_deg, lon_deg, alt_m},
            m_timestamp{ts} {}

        explicit GPS_Coordinates(const std::array<double,3>& lla,
                        std::chrono::milliseconds ts = std::chrono::milliseconds{0})
            : m_lla{lla},
            m_timestamp{ts} {}

        double latitude() const noexcept{
            return m_lla[0];
        }
        
        double longitude() const noexcept{
            return m_lla[1];
        }
        
        double altitude() const noexcept{
            return m_lla[2];
        }

        std::chrono::milliseconds timestamp() const noexcept{
            return m_timestamp;
        }
        
        
    }; 
} // namespace UAV::Software::Data_Types

#endif