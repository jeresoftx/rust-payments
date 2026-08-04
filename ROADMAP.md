# Roadmap

Ruta viva de `rust-payments`, curso aplicado de pasarelas de pago e integración
fintech de Jeresoft Academy.

No hay fechas límite: la corrección, la seguridad y el porqué documentado
prevalecen sobre la velocidad (RFC-0001 §1 y §2).

## Orden de construcción

1. Modelo de dominio, dinero y ciclo de vida de transacciones.
2. Protección de datos, idempotencia y adaptadores simulados.
3. Webhooks, conciliación, fraude y controles de operación.
4. Caso integrador sintético, auditoría de draft y revisión humana.

El [Project operativo de Rust Payments](https://github.com/users/jeresoftx/projects/30)
conserva la ruta crítica, los issues, milestones y bloqueadores; su vista
principal está agrupada por milestone. El detalle vive en
[plan/curso-rust-payments.md](plan/curso-rust-payments.md).

## Límites de autonomía

Una IA puede preparar documentación, modelos deterministas en memoria y
pruebas. Requieren decisión humana previa proveedores reales, SDKs de pago,
datos financieros o personales, dinero real, credenciales, firmas
criptográficas, requisitos regulatorios y afirmaciones de cumplimiento.
