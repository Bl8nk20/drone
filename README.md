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

* $u(t)$ = Stellgröße
* $e(t) = w(t) - y(t)$ = Regeldifferenz (Sollwert $w(t)$ minus Istwert $y(t)$)
* $K_P$ = Proportionalfaktor -> Reagiert direkt auf aktuelle abweichung
* $K_I$ = Integralfaktor -> Summiert vergangene Fehler, bleibende Abweichung reduzieren/eliminieren
* $K_D$ = Differentialfaktor -> Reagiert auf Änderungsrate des Fehlers, schnelle Schwankungen dämpfen

### Calibrierung mittels Ziegler-Nichols-Methode

1. $K_I = 0; K_D = 0$
2. $K_P$ schrittweise erhöhen -> (kritische Verstärkung $K_U$)
3. Schwingungsperiode bestimmen $T_U$
4. PID-Parameter berechnen:  
    * $K_P = 0.6 * K_U$
    * $K_I = 2 * \frac{K_P}{T_U}$
    * $K_D = K_P * \frac{T_U}{8}$

Möglicherweise Implementierung eines Weiteren, Automatische Abstimmungsmethode  

* Gradientenabstieg
* Partikelschwarmoptimierung

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
            "Kd" : double positiv -> möglichst klein,
            "Tu" : double,
            "Ku" : double
        }
    ]
}
```
