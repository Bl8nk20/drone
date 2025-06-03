#ifndef UAV_SOFTWARE_IDATA_GPS_H
#define UAV_SOFTWARE_IDATA_GPS_H

#pragma once

#include <array>
#include <cmath>
#include <string>
#include <sstream>
#include <iomanip>
#include <chrono>

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

        /**
         * @brief 
         * 
         * @return true 
         * @return false 
         */
        bool _isValid() const;
        
        /**
         * @brief 
         * 
         * @param other 
         * @return float 
         */
        double distanceTo(const GPS_Coordinates& other) const;


        public:
        /**
         * @brief Construct a new gps coordinates object
         * 
         */
        GPS_Coordinates() = default;
        /**
         * @brief Construct a new gps coordinates object
         * 
         * @param lat_deg 
         * @param lon_deg 
         * @param alt_m 
         * @param ts 
         */
        GPS_Coordinates(double lat_deg,
                        double lon_deg,
                        double alt_m = 0.0,
                        std::chrono::milliseconds ts = std::chrono::milliseconds{0})
            : m_lla{lat_deg, lon_deg, alt_m},
            m_timestamp{ts} {}

        /**
         * @brief Construct a new gps coordinates object
         * 
         * @param lla 
         * @param ts 
         */
        explicit GPS_Coordinates(const std::array<double,3>& lla,
                        std::chrono::milliseconds ts = std::chrono::milliseconds{0})
            : m_lla{lla},
            m_timestamp{ts} {}

        /**
         * @brief 
         * 
         * @return double 
         */
        double latitude() const noexcept{
            return m_lla[0];
        }
        
        /**
         * @brief 
         * 
         * @return double 
         */
        double longitude() const noexcept{
            return m_lla[1];
        }
        
        /**
         * @brief 
         * 
         * @return double 
         */
        double altitude() const noexcept{
            return m_lla[2];
        }

        /**
         * @brief 
         * 
         * @return std::chrono::milliseconds 
         */
        std::chrono::milliseconds timestamp() const noexcept{
            return m_timestamp;
        }
        
        /**
         * @brief Get the Coords object
         * 
         * @return std::array<double, 3> 
         */
        std::array<double, 3> getCoords() const noexcept{
            return m_lla;
        }

        /**
         * @brief 
         * 
         * @return true 
         * @return false 
         */
        bool isValid() const {
            return _isValid();
        }

        std::string toString() const;

        
    }; 

    /**
     * @brief 
     * 
     * @return true 
     * @return false 
     */
    bool GPS_Coordinates::_isValid() const{
            return std::abs(m_lla[0]) <= 90.0 &&
            std::abs(m_lla[1]) <= 180.0;
    }

    /**
     * @brief 
     * 
     * @param other 
     * @return double 
     */
    double GPS_Coordinates::distanceTo(const GPS_Coordinates& other) const{
        const double lat1_rad = latitude() * M_PI / 180.0;
        const double lon1_rad = longitude() * M_PI / 180.0;
        const double lat2_rad = other.latitude() * M_PI / 180.0;
        const double lon2_rad = other.longitude() * M_PI / 180.0;

        const double dlat = lat2_rad - lat1_rad;
        const double dlon = lon2_rad - lon1_rad;

        const double a = std::sin(dlat / 2) * std::sin(dlat / 2) +
                        std::cos(lat1_rad) * std::cos(lat2_rad) *
                        std::sin(dlon / 2) * std::sin(dlon / 2);

        const double c = 2 * std::atan2(std::sqrt(a), std::sqrt(1 - a));
        const double ground_distance = kEarthRadius_m * c;
        const double dz = altitude() - other.altitude();

        return std::sqrt(ground_distance * ground_distance + dz * dz);
    }

    std::string GPS_Coordinates::toString() const {
        std::ostringstream oss;
        oss << std::fixed << std::setprecision(6)
            << "Lat: " << latitude()
            << ", Lon: " << longitude()
            << ", Alt: " << altitude()
            << ", Timestamp: " << timestamp().count() << "ms";
        return oss.str();
}


} // namespace UAV::Software::Data_Types

#endif