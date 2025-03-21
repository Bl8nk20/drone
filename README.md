# drone

a simple sub 250g UAV to legally fly in Germany

## Names

Command Center == Laptop / Remote Controller

## Keeping in Mind

* Lightweight UAV
* Running on a Banana Pi -> fast and low Level

## Design Patterns to consider

* Singleton
* Command -> for Command Center Input
* maybe I will need multiple Adapter
* Bridge

## PiD-Filter

$$u(t) = K_p * e(t) + K_I *\int_1^4 e(t) dt + K_D * \frac{de(t)}{dt}$$

, wobei:

* $u(t)$ = Output Signal to engine
* $e(t) = w(t) - y(t)$ = Difference ($w(t)$ minus $y(t)$)
* $K_P$ = Proportionalfactor -> Reacts directly to current error
* $K_I$ = Integralfaktor -> Sums up past errors, reduces/eliminate current error
* $K_D$ = Differentialfaktor -> Reacts to change of rate of error, quick swings dampening

### Calibrierung mittels Ziegler-Nichols-Methode

1. $K_I = 0; K_D = 0$
2. $K_P$ increase stepwise -> (critical increasing $K_U$)
3. Swingingperiod $T_U$
4. PID-Parameter calculating:  
    * $K_P = 0.6 * K_U$
    * $K_I = 2 * \frac{K_P}{T_U}$
    * $K_D = K_P * \frac{T_U}{8}$

Optionally adding one other Automatic Leveling algorithm like:

* Gradient step down
* PSO

## Structure of the Files

* Reading of Last used PiD-Filter variables
* json Format

``` Json
{
    "PiD-Filter_Settings" :
    [
        {
            "Kp" : double positiv,
            "Ki" : double positiv, 
            "Kd" : double positiv -> best if small
        }
    ]
}
```
