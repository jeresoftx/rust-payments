# Estado del draft

Este repositorio contiene un **curso en draft**. Su propósito es practicar el
razonamiento de integración alrededor de pagos, no operar una pasarela ni
procesar transacciones reales.

## Contrato del curso

- Todos los montos, comercios, referencias y eventos son sintéticos.
- El código se ejecuta completamente en memoria y no abre conexiones de red.
- No se usan tarjetas, cuentas, datos personales, credenciales, secretos ni
  SDKs de proveedores.
- Las referencias a PCI-DSS, 3-D Secure, SCA y fraude describen límites y
  vocabulario; no constituyen cumplimiento ni asesoría regulatoria.
- Cada capítulo presenta concepto, problema, alternativas y justificación
  antes de su implementación, conforme a RFC-0001 §2.

## Qué significa `draft`

El contenido puede ser leído, ejecutado y auditado técnicamente, pero aún
requiere revisión humana editorial, técnica y de seguridad. Por ello no se
marca como `reviewed` ni `published` (RFC-0001 §20).

## Decisión de diseño

**Concepto:** enseñar pagos exige que los estados, errores y fronteras sean
observables.

**Problema:** un ejemplo conectado a un proveedor real convertiría un curso
en una fuente de credenciales, costos, riesgo y afirmaciones difíciles de
verificar.

**Alternativas:** depender de un SDK real, construir un procesador simulado de
gran escala o modelar contratos pequeños, deterministas y explícitos.

**Justificación:** se elige el modelo determinista. Conserva las decisiones
ingenieriles relevantes y permite pruebas reproducibles sin ampliar el riesgo
ni el alcance del curso.
