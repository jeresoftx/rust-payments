# 07. Webhooks y cambios de estado asíncronos

## Concepto

Un webhook comunica que otro sistema observó un cambio. Puede llegar tarde,
duplicado o fuera de orden. El receptor necesita correlacionar el evento,
deduplicarlo y decidir si cambia un estado local.

## Problema

Confiar en el orden de llegada o aplicar cada evento dos veces crea estados
imposibles. Modelar una firma real añadiría secretos y criptografía que este
curso no posee ni certifica.

## Alternativas y justificación

Polling simplifica el consumidor pero no elimina discrepancias; aceptar todo
evento es frágil. Se usa un receptor en memoria que conserva identificadores
vistos y permite aplicar solo eventos con una secuencia no regresiva. La firma
real queda explícitamente fuera de alcance.

## Calidad

No aplica benchmark ni property testing: el contrato se demuestra con eventos
duplicados y tardíos deterministas, no con un supuesto de red real.
