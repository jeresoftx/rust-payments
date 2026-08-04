# Plan de curso: Rust Payments

**Estado:** draft completo; revisión humana pendiente.

**Fuente de alcance:** RFC-0001 §10, "Pasarelas de pago".

**Seguimiento operativo:** [Rust Payments — Plan operativo](https://github.com/users/jeresoftx/projects/30),
con issues asignados, milestones por capítulo y vista principal agrupada por
milestone.

## Propósito

Construir un curso aplicado de diez capítulos sobre integración y operación de
pasarelas de pago con Rust. El foco es modelar flujos correctos y auditables,
no procesar dinero ni conectarse a proveedores reales.

## Decisión de alcance

**Concepto:** una integración de pagos coordina estados, dinero, proveedores,
eventos asíncronos y auditoría; no es solo una solicitud HTTP.

**Problema:** tratar pagos como una llamada de éxito o error oculta duplicados,
redondeos, estados tardíos, fraude, discrepancias y responsabilidades de
seguridad.

**Alternativas:** integrar un proveedor real desde el primer capítulo,
construir un procesador propio o usar modelos sintéticos y deterministas antes
de cualquier dependencia externa.

**Justificación:** el primer draft usará contratos en memoria, dinero
sintético y adaptadores simulados. Los proveedores reales se estudiarán como
casos de integración, solo después de justificar credenciales, licencias,
seguridad, pruebas y cumplimiento aplicable.

## Reglas de implementación

- Un issue equivale a una rama, un commit principal y un PR.
- Cada capítulo contiene explicación, diagrama Mermaid, ejemplos, ejercicios,
  soluciones, pruebas y benchmark o decisión documentada de no aplicar.
- No se usa `unsafe`, dinero real, datos de tarjeta, credenciales, SDKs de
  proveedores ni dependencias externas no triviales sin autorización humana.
- Los montos nuevos se modelan en unidades menores enteras y moneda explícita;
  no se usan flotantes para dinero.
- Ningún capítulo se marca como `reviewed` o `published` sin revisión humana.

## Capítulos y dependencias

### 01. Modelo de una pasarela y sus actores — draft

- [x] Explicar comercio, adquirente, emisor, red y proveedor.
- [x] Modelar fronteras de responsabilidad y estados sintéticos.
- [x] Comparar integración de proveedor, orquestación propia y procesador.

### 02. Dinero, monedas y redondeo — draft

- [x] Modelar montos en unidades menores y códigos de moneda explícitos.
- [x] Explicar por qué los flotantes no representan dinero correctamente.
- [x] Probar conversiones, redondeos y límites sintéticos.

### 03. Ciclo de vida de una transacción — draft

- [x] Modelar autorización, captura, liquidación, reverso y reembolso.
- [x] Comparar máquina de estados explícita contra estados implícitos.
- [x] Probar transiciones válidas e inválidas sin dinero real.

### 04. Tokenización, PCI-DSS y límites de datos — draft

- [x] Explicar tokenización y la frontera de responsabilidad PCI-DSS.
- [x] Modelar referencias sintéticas sin datos de tarjeta.
- [x] Documentar lo que el curso no certifica ni almacena.

### 05. Idempotencia y reintentos seguros — draft

- [x] Modelar una llave de idempotencia y resultados repetibles.
- [x] Comparar reintento ciego, deduplicación y saga compensatoria.
- [x] Probar que un duplicado sintético no crea un segundo cobro.

### 06. Adaptadores de proveedores — draft

- [x] Diseñar un puerto de proveedor y adaptadores simulados.
- [x] Comparar contratos internos con SDKs y APIs externas.
- [x] Documentar credenciales, versionado y errores como límites de integración.

### 07. Webhooks y cambios de estado asíncronos — draft

- [x] Modelar recepción, validación sintética y orden no garantizado de eventos.
- [x] Comparar polling, webhook y reconciliación posterior.
- [x] Probar deduplicación y transiciones tardías deterministas.

### 08. Conciliación, auditoría y discrepancias — draft

- [x] Modelar un libro de eventos sintético y discrepancias explícitas.
- [x] Comparar estado de proveedor, estado interno y evidencia auditable.
- [x] Probar flujos de conciliación y decisiones de reparación.

### 09. Fraude, 3-D Secure y SCA — draft

- [x] Explicar señales, autenticación reforzada y decisiones de riesgo.
- [x] Modelar decisiones sintéticas sin puntaje ni datos personales reales.
- [x] Documentar límites de antifraude y cumplimiento.

### 10. Caso integrador de operación fintech — draft

- [x] Unir orden, pago sintético, idempotencia, webhook y conciliación.
- [x] Hacer visibles observabilidad, auditoría y fallas parciales.
- [x] Cerrar con un caso determinista, sin proveedor, dinero ni datos reales.

## Auditoría y revisión

- [x] Auditar crate, manifest, diagramas, ejemplos, benchmarks y límites de
      seguridad, privacidad y licencias.
- [ ] Solicitar revisión humana editorial, técnica y de seguridad antes de
      cambiar el estado del draft.

## Fuera de alcance activo

No se activan proveedores reales, cobros, datos de tarjeta, tokens de
producción, firmas criptográficas, cumplimiento normativo, KYC, antifraude real
ni infraestructura financiera mientras no exista decisión humana y plan nuevo.
