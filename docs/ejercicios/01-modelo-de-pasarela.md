# Ejercicio 01: hacer visible una frontera

## Situación

Un equipo quiere imprimir un resultado después de registrar una intención
sintética. El requisito no es cobrar ni hablar con una API: solo conservar la
referencia local y mostrar el estado que el modelo permite observar.

## Objetivo

Completa un programa que cree `IntentId`, construya `SyntheticIntent::received`
y ejecute `evaluate`.

## Restricciones

- No agregues dependencias.
- No modeles datos de tarjeta, cuentas, importes ni una red.
- Trata `Accepted` como un resultado local del ejemplo, no como un cobro.

## Pista

`IntentId::new` devuelve `Result`; una referencia vacía no puede correlacionar
un evento de prueba.

## Solución comentada

La [solución](../../examples/01_gateway.rs) mantiene el contrato mínimo:
construye una referencia sintética, la recibe y la evalúa en memoria. El texto
impreso es evidencia del ejemplo, no un recibo financiero.
