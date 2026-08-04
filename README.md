# Rust Payments

Curso aplicado de Jeresoft Academy para estudiar la integración y operación de
pasarelas de pago con Rust. Su foco es corrección, auditoría, idempotencia y
límites de seguridad: manejar dinero no admite resultados "casi correctos"
(RFC-0001 §10).

El curso enseña desde la experiencia de **integrar y operar** proveedores, no
de construir un procesador de pagos ni una red de tarjetas desde cero. Cada
capítulo distingue ese límite con honestidad (RFC-0001 §1 y §20).

## Estado

El repositorio está en planificación. El [plan operativo](plan/curso-rust-payments.md)
define los diez capítulos y el [Project operativo de GitHub](https://github.com/users/jeresoftx/projects/30)
los convierte en trabajo trazable antes de implementar contenido.

## Temario base

| # | Capítulo | Estado |
| --- | --- | --- |
| 01 | Modelo de una pasarela y sus actores | draft |
| 02 | Dinero, monedas y redondeo | draft |
| 03 | Ciclo de vida de una transacción | planned |
| 04 | Tokenización, PCI-DSS y límites de datos | planned |
| 05 | Idempotencia y reintentos seguros | planned |
| 06 | Adaptadores de proveedores | planned |
| 07 | Webhooks y cambios de estado asíncronos | planned |
| 08 | Conciliación, auditoría y discrepancias | planned |
| 09 | Fraude, 3-D Secure y SCA | planned |
| 10 | Caso integrador de operación fintech | planned |

## Prerrequisitos y lugar en el camino

`rust-payments` es un curso complementario de dominio aplicado. Conecta, sin
duplicarlos, los fundamentos canónicos de:

- `rust-crypto` para tokenización y criptografía.
- `rust-security` para límites de seguridad y threat modeling.
- `rust-api-design` para contratos, errores y webhooks.
- `rust-distributed-systems` para idempotencia, reintentos y consistencia.
- `rust-devops` para operación, auditoría y observabilidad.

## Límites de seguridad

- No se usan tarjetas, dinero, cuentas, credenciales ni proveedores reales.
- No se declara cumplimiento PCI-DSS, 3-D Secure, SCA, KYC o antifraude.
- No se almacenan datos personales, financieros ni secretos.
- Cualquier dependencia de proveedor, SDK, firma criptográfica o integración
  externa requiere justificación escrita y revisión humana previa.

## Verificación base

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- [ROADMAP.md](ROADMAP.md) ordena el recorrido sin fechas artificiales.
- [plan/curso-rust-payments.md](plan/curso-rust-payments.md) es el plan vivo
  y fuente de los issues del Project.
- [Rust Payments — Plan operativo](https://github.com/users/jeresoftx/projects/30)
  mantiene la vista principal agrupada por milestone.
- [AGENTS.md](AGENTS.md) establece límites para personas y agentes.
- [LICENSE.md](LICENSE.md) explica la doble licencia de código y contenido.

Ningún capítulo se marcará como `reviewed` o `published` sin revisión humana
(RFC-0001 §20).
