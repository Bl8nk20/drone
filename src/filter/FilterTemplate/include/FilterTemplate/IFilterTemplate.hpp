#ifndef UAV_SOFTWARE_IFILTERTEMPLATE_H
#define UAV_SOFTWARE_IFILTERTEMPLATE_H

#include <array>

namespace UAV::Software::Filter{
    
    template <typename T> 
    class IFilterTemplate{
        public:
        IFilterTemplate();
        /**
         * @brief 
         * 
         */

        protected:
        virtual void tune_filter() = 0 const;
    };
} // namespace UAV::Software::Filter

#endif