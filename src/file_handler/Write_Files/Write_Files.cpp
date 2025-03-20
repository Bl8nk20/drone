# include "Write_Files/Write_Files.hpp"

using namespace UAV::Software::File_Handler;
using json = nlohmann::json;

Write_Files::Write_Files(const std::string& FileName) : FileName(FileName){}

void Write_Files::write_contents(const std::map<std::string, double>& contents) const{

}