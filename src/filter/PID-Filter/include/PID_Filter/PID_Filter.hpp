#ifndef UAV_SOFTWARE_PID_FILTER_H
#define UAV_SOFTWARE_PID_FILTER_H

#include <map>
#include "FilterTemplate/IFilterTemplate.hpp"

namespace UAV::Software::Filter{

    /**
     * @brief 
     * PID-Filter uses the standard equation:
     * u(t) = K_p * (e(t) + 1/T_i * integral{lower=0, upper=t}e(tau)dtau + T_d * d/(dt)*e(t))
     * where:
     * u(t) = Output signal after Filtering
     * e(t) = Difference of Current_Value and Targeted_Value
     * K_p = Proportialfactor
     * T_i = integration time constant = K_p / K_i
     * T_d = derivative time constant = K_d / K_p
     */
    class PID_Filter : IFilterTemplate<double>{
        public:
        /**
         * @brief Construct a new pid filter object
         * 
         * @param last_flight_values 
         */
        PID_Filter(const std::map<std::string, double>& last_flight_values);

        /**
         * @brief Construct a new pid filter object
         * 
         * @param K_u 
         */
        PID_Filter(const double& K_u);

        /**
         * @brief 
         * 
         */
        void tune_filter() override;

        /**
         * @brief 
         * 
         * @param target_value 
         * @param current_value 
         */
        void update_et(const double& target_value, const double& current_value) const;

        /**
         * @brief 
         * 
         * @param target_value 
         * @param current_value 
         */
        void update_ut(const double& target_value, const double& current_value) const;
        /**
         * @brief Get the ut object
         * 
         * @return double 
         */
        double get_ut() const;

        /**
         * @brief Get the et object
         * 
         * @return double 
         */
        double get_et() const;

        /**
         * @brief Get the last params object
         * 
         * @return std::map<std::string, double> 
         */
        std::map<std::string, double> get_last_params();
        
        private:
        const std::map<std::string, double> m_last_flight_values;
        mutable double u_t;
        mutable double e_t;
        mutable double K_i;
        mutable double K_d;

        /**
         * @brief 
         * 
         */
        void calculate_ut(const double& target_value, const double& current_value) const;

        /**
         * @brief 
         * 
         */
        void calculate_et(const double& target_value, const double& current_value) const;

        /**
         * @brief 
         * 
         */
        void calculate_Ki() const;

        /**
         * @brief 
         * 
         */
        void calculate_Kd() const;
    };
}

#endif