#ifndef UAV_SOFTWARE_FILE_HANDLER_H
#define UAV_SOFTWARE_FILE_HANDLER_H

#include <string>
#include <map>
#include <iostream>
#include <fstream>
#include <nlohmann/json.hpp>

namespace UAV::Software::File_Handler{
    /**
     * @brief 
     * 
     */
    class File_Handler{
        public:
        File_Handler(const std::string& Filename) : m_Filename(Filename){}

        std::map<std::string, double> get_written_content() const;
        void write_last_content(const std::map<std::string, double>& last_data) const;
        private:
        const std::string m_Filename;
        std::map<std::string, double> read_files() const;
    };
}

#endif