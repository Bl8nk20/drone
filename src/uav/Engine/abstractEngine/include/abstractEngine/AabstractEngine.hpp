#pragma once

namespace UAV::Engine
{
    class AabstractEngine{
    private:
        static constexpr double speedRPM = 0.0;
    protected:
        static constexpr void setSpeed(const double& rpm){
            this->speedRPM = rpm;
        };
    public:
        AabstractEngine() = default;
        virtual ~AabstractEngine() = default;
        virtual void start() = 0;
        virtual void stop() = 0;
    };
}; // namespace UAV::Engine
