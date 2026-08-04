//! Modelos pedagógicos para integrar pasarelas de pago con Rust.
//!
//! El crate empieza con contratos sintéticos y verificables. No procesa dinero
//! real, tarjetas, credenciales ni peticiones a proveedores.

#![forbid(unsafe_code)]

pub mod gateway;
pub mod idempotency;
pub mod lifecycle;
pub mod money;
pub mod provider;
pub mod token;
pub mod webhook;

/// Declara que el curso todavía está en planificación.
pub const fn course_status() -> &'static str {
    "draft"
}

#[cfg(test)]
mod tests {
    use super::course_status;

    #[test]
    fn crate_declara_el_estado_de_draft() {
        assert_eq!(course_status(), "draft");
    }
}
