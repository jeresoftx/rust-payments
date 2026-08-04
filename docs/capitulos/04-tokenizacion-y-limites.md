# 04. Tokenización, PCI-DSS y límites de datos

## Concepto

Un token sustituye una referencia sensible por otra referencia con un ámbito
controlado. La tokenización no convierte por sí misma un sistema en conforme:
la responsabilidad depende de datos, arquitectura, contratos y operación.

## Problema

Guardar o propagar información sensible fuera de su frontera aumenta el daño
posible y vuelve opaca la responsabilidad. Un ejemplo didáctico que inventa
un número de tarjeta también enseña la frontera equivocada.

## Alternativas y justificación

Se descarta almacenar datos sensibles o construir criptografía propia. El
curso modela solo referencias sintéticas con propósito y vencimiento lógico.
Así permite hablar de minimización de datos sin afirmar cumplimiento PCI-DSS
ni representar un token emitido por un proveedor.

## Calidad

No aplica benchmark ni property testing por ahora: las invariantes son finitas
y se verifican con pruebas de propósito, vencimiento y ausencia de secreto.
