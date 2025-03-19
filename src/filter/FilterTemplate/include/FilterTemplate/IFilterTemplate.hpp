#ifndef UAV_SOFTWARE_IFILTERTEMPLATE_H
#define UAV_SOFTWARE_IFILTERTEMPLATE_H

#include <array>

namespace UAV::Software::Filter{
    
    template <typename T> 
    class IFilterTemplate{
        public:
        IFilterTemplate(const std::string& t_Filename);
        virtual std::array<T, 3> get_FilteredData() = 0 const;
        void startup() const;
        void shutdown() const;
        void tune_filter() const;

        private:
        static constexpr std::string FilterFilename;
    };
} // namespace UAV::Software::Filter

#endif