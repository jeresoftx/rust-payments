# 08. Conciliación, auditoría y discrepancias

## Concepto

Conciliar es comparar evidencia de dos fuentes y decidir si coinciden. Un
registro interno y un resultado de proveedor pueden diferir aun cuando ambos
sean plausibles; la discrepancia es información que debe permanecer visible.

## Problema

Tomar un solo estado como verdad borra la posibilidad de investigar, reparar o
explicar un desfase. Intentar modelar un libro financiero real ampliaría el
curso hacia contabilidad y operación regulada.

## Alternativas y justificación

Se descarta “corregir” automáticamente cualquier diferencia. El modelo
compara dos resultados sintéticos y produce `Match` o `Mismatch` con la
referencia local. Así enseña evidencia auditable sin afirmar conciliación de
producción.

## Calidad

No aplica benchmark ni property testing: la matriz de coincidencia y
discrepancia es pequeña y queda cubierta por pruebas explícitas.
