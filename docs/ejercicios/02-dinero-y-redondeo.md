# Ejercicio 02: el residuo es una decisión

Divide `1_000` unidades menores sintéticas de MXN entre tres partes. Primero
intenta `RejectRemainder`; después compara el resultado de `AwayFromZero`.

No conviertas a `f64`: el objetivo es observar que el residuo exige una regla,
no una aproximación silenciosa. La [solución](../../examples/02_money.rs)
declara la regla antes de imprimir el resultado.
