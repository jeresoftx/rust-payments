# 06. Adaptadores de proveedores

## Concepto

Un puerto interno expresa lo que el dominio necesita; un adaptador traduce ese
contrato hacia un proveedor. El curso conserva el puerto estable y usa
adaptadores simulados para estudiar respuestas, errores y versionado.

## Problema

Si el resto del sistema conoce cada SDK externo, un cambio de proveedor se
propaga por todo el código. Usar directamente un cliente de red también
introduciría credenciales, costos y un comportamiento que este curso no puede
verificar.

## Alternativas y justificación

Se descartan los SDKs reales y un adaptador universal con demasiadas opciones.
Se elige un trait pequeño con dos implementaciones simuladas. La comparación
enseña la traducción de contratos sin prometer que los proveedores reales sean
equivalentes.

## Calidad

No aplica benchmark: medir dos structs en memoria no representa latencia ni
contratos externos. Las pruebas directas cubren la respuesta estable del puerto
sin agregar property testing ni dependencias.
