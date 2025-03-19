#ifndef UAV_SOFTWARE_IDATA_GPS_COORDINATES_H
#define UAV_SOFTWARE_IDATA_GPS_COORDINATES_H

#include <array>
#include <string>

namespace UAV::Software::Data_Types{
    
    class GPS_Coordinates{
        private:
        static std::array<double, 3> Lat_Lon_Decimal;
        static std::array<std::string, 3> Lat_Lon_Degree;

        double convert_to_Barometric_Pressure(const double& height) const;

        public:
        GPS_Coordinates(const double& Lat, const double& Lon, const double& height);
        GPS_Coordinates(const std::string& Lat, const std::string& Lon, const std::string& height);

        std::array<double, 3> get_Lat_Lon_Decimal() const;
        std::array<std::string, 3> get_Lat_Lon_Degree() const;
        double get_Barometric_Pressure() const;
        double get_Height() const;
        
        void set_Lat_Lon_Decimal(const std::array<double, 3>& Lat_Lon_Decimal) const;
        void set_Lat_Lon_Degree(const std::array<std::string, 3>& Lat_Lon_Degree) const;
        void set_Height(const double& height) const;     
    };

/*----------            Constructor         -------------*/
GPS_Coordinates::GPS_Coordinates(const double& Lat, const double& Lon, const double& height){
    Lat_Lon_Decimal[0] = Lat;
    Lat_Lon_Decimal[1] = Lon;
    Lat_Lon_Decimal[2] = height;
}

GPS_Coordinates::GPS_Coordinates(const std::string& Lat, const std::string& Lon, const std::string& height){
    Lat_Lon_Degree[0] = Lat;
    Lat_Lon_Degree[1] = Lon;
    Lat_Lon_Degree[2] = height;
}

/*----------            Getter         -------------*/

std::array<double, 3> GPS_Coordinates::get_Lat_Lon_Decimal() const{
    return Lat_Lon_Decimal;
}

std::array<std::string, 3> GPS_Coordinates::get_Lat_Lon_Degree() const{
    return Lat_Lon_Degree;
}

double GPS_Coordinates::get_Barometric_Pressure() const{

    double barometric_pressure = convert_to_Barometric_Pressure(Lat_Lon_Decimal[2]);

    return barometric_pressure;
}

double GPS_Coordinates::get_Height() const{
    return Lat_Lon_Decimal[2];
}

/*----------            Setter         -------------*/

void GPS_Coordinates::set_Lat_Lon_Decimal(const std::array<double, 3>& Lat_Lon_Decimal) const{
    this->Lat_Lon_Decimal = Lat_Lon_Decimal;
}
void GPS_Coordinates::set_Lat_Lon_Degree(const std::array<std::string, 3>& Lat_Lon_Degree) const{
    this->Lat_Lon_Degree = Lat_Lon_Degree;
}
void GPS_Coordinates::set_Height(const double& height) const{
    this->Lat_Lon_Decimal[2] = height;
}

/*----------            Calculations         -------------*/

double GPS_Coordinates::convert_to_Barometric_Pressure(const double& height) const{
    double Ph; // Air Pressure at altitude
    double Pu = 1013.25; // Environment Pressure (0m abve sea level)
    double a = 0.0065; // Temperature Gradient
    double Th = 288.15; // Standerd Temperature

    Ph = Pu * (1 - (a * height)/Th);

    return Ph;
}
} // namespace UAV::Software::Data_Types
#endif