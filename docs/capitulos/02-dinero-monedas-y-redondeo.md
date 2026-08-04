# 02. Dinero, monedas y redondeo

## Concepto

Un monto no es un número decimal aislado. Para este curso es una cantidad en
unidad menor más una moneda explícita: `1250 MXN` representa mil doscientos
cincuenta centavos, no un `12.50` flotante.

## Problema

Los flotantes aproximan fracciones binarias. Esa aproximación puede volver
impredecible una suma, comparación o regla de redondeo. También es incorrecto
sumar valores de monedas diferentes sin una conversión que este curso no
pretende resolver.

## Alternativas

| Alternativa | Riesgo |
| --- | --- |
| `f64` con dos decimales | No representa exactamente todos los decimales. |
| Decimal externo | Puede ser útil en otro alcance, pero añade una dependencia y reglas que aquí no se necesitan. |
| Entero de unidades menores y moneda | Hace explícita la precisión y rechaza mezclas inválidas. |

## Justificación

Se usa `i64` para unidades menores sintéticas y un código de moneda cerrado.
La división que produce residuo requiere una regla de redondeo visible. El
modelo no convierte divisas, no calcula impuestos ni representa saldos reales.

## Calidad

**Benchmark:** no aplica; las operaciones son aritmética local y no describen
rendimiento financiero. **Property testing:** no agrega dependencia mientras
las combinaciones finitas de moneda y redondeo se cubran con pruebas de tabla.

Las pruebas del crate cubren una mezcla inválida de monedas y un residuo que
debe fallar o redondearse según una política visible. Esa evidencia es más
útil que una medición de nanosegundos aislada o entradas generadas que oculten
la regla pedagógica.

## Implementación

El módulo `money` rechaza monedas distintas, conserva unidades enteras y hace
que toda división declare si trunca, redondea hacia arriba o rechaza el residuo.
