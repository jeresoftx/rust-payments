//! Modelos pedagógicos para integrar pasarelas de pago con Rust.
//!
//! El crate empieza con contratos sintéticos y verificables. No procesa dinero
//! real, tarjetas, credenciales ni peticiones a proveedores.

#![forbid(unsafe_code)]

/// Declara que el curso todavía está en planificación.
pub const fn course_status() -> &'static str {
    "planned"
}

#[cfg(test)]
mod tests {
    use super::course_status;

    #[test]
    fn crate_declara_el_estado_planeado() {
        assert_eq!(course_status(), "planned");
    }
}
