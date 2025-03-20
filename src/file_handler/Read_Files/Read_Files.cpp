#include "Read_Files/Read_Files.hpp"

using namespace UAV::Software::File_Handler;

Read_Files::Read_Files(const std::string& FileName) : FileName(FileName){
}

std::array<std::string, 3> Read_Files::get_reading() const{
    std::ifstream file_content;


std::array<std::string, 3> Read_Files::read_contents() const{

}