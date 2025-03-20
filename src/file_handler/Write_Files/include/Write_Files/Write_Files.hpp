#ifndef UAV_SOFTWARE_FILE_WRITER_H
#define UAV_SOFTWARE_FILE_WRITER_H

#include <string>
#include <map>
#include <fstream>
#include <nlohmann/json.hpp>

namespace UAV::Software::File_Handler{
    /**
     * @brief 
     * 
     */
    class Write_Files{
        public:
        /**
         * @brief Construct a new Read_Files object
         * 
         * @param FileName 
         */
        Write_Files(const std::string& FileName);

        void write_contents(const std::map<std::string, double>& contents) const;
        private:
        const std::string FileName;


    };
} // namespace UAV::Software::File_Handler

#endif