# AGENTS.md

Este repositorio pertenece a Jeresoft Academy y se rige por el Manual
Fundacional RFC-0001.

## Objetivo

Enseñar cómo integrar y operar pasarelas de pago con Rust, haciendo visibles
los estados, las fallas y las decisiones que protegen dinero y datos. El curso
no construye un procesador de pagos ni afirma experiencia fuera de la
integración y operación de proveedores (RFC-0001 §1 y §10).

## Antes de escribir código

Para cada cambio no trivial, documenta este orden (RFC-0001 §2):

1. Concepto.
2. Problema.
3. Alternativas.
4. Justificación.
5. Implementación.

## Seguridad, datos y dependencias

- Rust idiomático, rustfmt y Clippy sin advertencias; `unsafe` está prohibido.
- No agregar dependencias no triviales sin justificación escrita y autorización
  humana.
- No usar proveedores, SDKs, tarjetas, cuentas, dinero, credenciales, secretos
  ni webhooks reales sin autorización humana explícita.
- No declarar cumplimiento PCI-DSS, 3-D Secure, SCA, KYC o antifraude sin
  evidencia, alcance y revisión humana.
- El dinero se modela con valores sintéticos; no se debe usar `f32` ni `f64`
  para representar montos en nuevos contratos del curso.

## Límites canónicos

- Criptografía y tokenización: `rust-crypto`.
- Seguridad y threat modeling: `rust-security`.
- Contratos HTTP, errores y webhooks: `rust-api-design`.
- Idempotencia, reintentos y consistencia: `rust-distributed-systems`.
- Operación, auditoría y observabilidad: `rust-devops`.

`rust-payments` integra estos conocimientos en un dominio aplicado; no los
reexplica ni los sustituye.

## Flujo de GitHub

Antes de código de curso, el plan completo vive en milestones, issues y un
GitHub Project. Cada issue se asigna a `jeresoftx`, tiene milestone y labels.
Cada PR resuelve un issue, conserva la misma metadata, se agrega al mismo
Project y se verifica antes de revisión o fusión.

Flujo obligatorio: `1 issue -> 1 rama -> 1 commit principal -> 1 PR`.
No se fusiona un PR sin revisión humana, salvo autorización explícita de modo
autónomo con revisión diferida conforme a RFC-0001 §20.

## Verificación base

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
git diff --check
```
