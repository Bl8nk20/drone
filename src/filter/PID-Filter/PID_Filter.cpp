#include "PID_Filter/PID_Filter.hpp"

using namespace UAV::Software::Filter;

/*--------          Constructor         ------- */

PID_Filter::PID_Filter(const std::map<std::string, double>& last_flight_values) : m_last_flight_values(last_flight_values){}

PID_Filter::PID_Filter(const double& K_u){

}

/*--------          Getter         ------- */

double PID_Filter::get_ut(const double& target_value, const double& current_value) const{
    calculate_ut(target_value, current_value);
    return u_t;
}

double PID_Filter::get_last_et(const double& target_value, const double& current_value) const{
    calculate_last_et(target_value, current_value);
    return this->e_t[-1];
}

std::map<std::string, double> PID_Filter::get_last_params(){
    return m_last_flight_values;
}

/*--------          Tuning        ------- */

void PID_Filter::tune_filter(){
    // Manual Tuning, like 
}

/*--------          Calculations         ------- */

double PID_Filter::calculate_P_part() const{
    double P_part = m_last_flight_values.at("K_p") * e_t[-1];

    return P_part;
}

double PID_Filter::calculate_I_part() const{
    double T_i = m_last_flight_values.at("K_p") / m_last_flight_values.at("K_i");
    double sum = 0;
    
    for(const auto& et : e_t){
        sum += et;
    }

    double I_part = 1/ T_i * sum;
    return I_part;
}

double PID_Filter::calculate_D_part() const{
    double T_d = m_last_flight_values.at("K_d") / m_last_flight_values.at("K_p");
    double delt_time = 0.5;
    std::vector<double> diff_et;
    for(std::size_t i = 0; i < e_t.size(); i++){
        diff_et[i] = (e_t[i+1] - e_t[i])/delt_time;
    }    
    double D_part = T_d * diff_et[-1];
    return D_part;
}

void PID_Filter::calculate_ut(const double& target_value, const double& current_value) const{
    calculate_last_et(target_value, current_value);

    this->u_t = calculate_P_part() + calculate_I_part() + calculate_D_part();
}

void PID_Filter::calculate_last_et(const double& target_value, const double& current_value) const{

    this->e_t.push_back(target_value - current_value);
}