#include "Read_Files/Read_Files.hpp"

using namespace UAV::Software::File_Handler;
using json = nlohmann::json;

Read_Files::Read_Files(const std::string& FileName) : FileName(FileName){}

std::map<std::string, double> Read_Files::get_reading() const{

    return read_contents();
}

std::map<std::string, double> Read_Files::read_contents() const{
    std::ifstream f(FileName);
    json j = json::parse(f);
    
    std::map<std::string, double> m = j.at("PiD-Filter_Settings").get<std::map<std::string, double>>();
    
    return m;
}