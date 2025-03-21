#ifndef UAV_SOFTWARE_IFILTERTEMPLATE_H
#define UAV_SOFTWARE_IFILTERTEMPLATE_H

#include <array>

namespace UAV::Software::Filter{
    
    template <typename T> 
    class IFilterTemplate{
        public:
        IFilterTemplate();

        protected:
        /**
         * @brief 
         * 
         */
        void virtual tune_filter();
    };
} // namespace UAV::Software::Filter

#endif