#include "file_handler/file_handler.hpp"

using json = nlohmann::json;
using namespace UAV::Software::File_Handler;

std::map<std::string, double> File_Handler::get_written_content() const{
    return read_files();
}

void File_Handler::write_last_content(const std::map<std::string, double>& last_data) const{
    json o;
    o.at("PiD_Filter_Settings");

    std::ofstream f(m_Filename);
    for(const auto& content : last_data){
        o[content.first] = content.second;
    }

    f << o;
    f.close();
}

std::map<std::string, double> File_Handler::read_files() const{
    std::ifstream f(m_Filename);
    json j = json::parse(f);
    
    std::map<std::string, double> m = j.at("PiD_Filter_Settings").get<std::map<std::string, double>>();
    
    f.close();
    return m;
}