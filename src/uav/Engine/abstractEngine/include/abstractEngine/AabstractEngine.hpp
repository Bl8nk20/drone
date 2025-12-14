#pragma once

namespace UAV::Engine
{
    class AabstractEngine{
    private:
        static double speedRPM;
    protected:
        constexpr void setSpeed(const double& rpm){
            this->speedRPM = rpm;
        };

        constexpr double getSpeed(){
            return this->speedRPM;
        };
    public:
        AabstractEngine() = default;
        virtual ~AabstractEngine() = default;
        virtual void start() = 0;
        virtual void stop() = 0;
    };
}; // namespace UAV::Engine
