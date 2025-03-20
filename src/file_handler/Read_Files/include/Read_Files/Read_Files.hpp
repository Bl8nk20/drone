#ifndef UAV_SOFTWARE_FILE_READER_H
#define UAV_SOFTWARE_FILE_READER_H

#include <string>
#include <array>
#include <fstream>

namespace UAV::Software::File_Handler{
    /**
     * @brief 
     * 
     */
    class Read_Files{
        public:
        /**
         * @brief Construct a new Read_Files object
         * 
         * @param FileName 
         */
        Read_Files(const std::string& FileName);

        /**
         * @brief Get the reading object
         * 
         * @return std::array<std::string, 3> 
         */
        std::array<std::string, 3> get_reading() const;
        
        private:
        const std::string FileName;

        /**
         * @brief 
         * 
         * @return std::array<std::string, 3> 
         */
        std::array<std::string, 3> read_contents() const;
    };
} // namespace UAV::Software::File_Handler

#endif