# 05. Idempotencia y reintentos seguros

## Concepto

Una llave de idempotencia correlaciona intentos equivalentes. Si un cliente
reintenta porque perdió una respuesta, el sistema debe devolver el resultado
ya registrado en lugar de ejecutar otra operación sintética.

## Problema

Un reintento ciego confunde incertidumbre de red con una solicitud nueva. La
alternativa de no reintentar deja operaciones inconclusas sin una ruta clara de
recuperación. Ambas ocultan la decisión que protege la consistencia.

## Alternativas

| Alternativa | Consecuencia |
| --- | --- |
| Reintentar cada solicitud | Puede duplicar efectos. |
| Bloquear todo reintento | Pierde recuperabilidad. |
| Guardar llave y resultado | Hace visible la equivalencia y su resultado. |

## Justificación

El curso implementa un registro local en memoria. Una misma llave retorna su
primer resultado; una llave distinta representa una intención distinta. No hay
almacenamiento distribuido, caducidad ni coordinación con proveedores reales.

## Calidad

No aplica benchmark: el mapa en memoria no representa una base compartida. No
se agrega property testing porque las invariantes principales se cubren con
casos deterministas: misma llave, mismo resultado; llave nueva, nuevo registro.
