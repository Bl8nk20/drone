#include "PID_Filter/PID_Filter.hpp"

using namespace UAV::Software::Filter;

/*--------          Constructor         ------- */

PID_Filter::PID_Filter(){

}

PID_Filter::PID_Filter(const double& target_value, const double& current_value){

}

PID_Filter::PID_Filter(const std::map<std::string, double>& last_flight_values){

}

PID_Filter::PID_Filter(const double& K_u){

}

/*--------          Getter         ------- */

double PID_Filter::get_ut() const{
    return u_t;
}

double PID_Filter::get_et() const{
    return e_t;
}

std::map<std::string, double> PID_Filter::get_last_params(){
    return m_last_flight_values;
}

/*--------          Setter         ------- */

void PID_Filter::set_last_params(std::map<std::string, double>& last_params){
    this->m_last_flight_values = last_params;
}

/*--------          Tuning and Keeping alive         ------- */

void PID_Filter::tune_filter(){

}

void PID_Filter::update_et(const double& target_value, const double& current_value) const{
    calculate_et(target_value, current_value);
}
void PID_Filter::update_ut(const double& target_value, const double& current_value) const{
    calculate_ut();
}

/*--------          Calculations         ------- */

void PID_Filter::calculate_ut() const{

}

void PID_Filter::calculate_et() const{

}

void PID_Filter::calculate_Ki() const{

}

void PID_Filter::calculate_Kd() const{

}