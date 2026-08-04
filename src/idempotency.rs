//! Registro en memoria para demostrar idempotencia con datos sintéticos.

use std::collections::HashMap;

/// Llave local que correlaciona intentos equivalentes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Crea una llave no vacía para una demostración local.
    pub fn new(value: impl Into<String>) -> Result<Self, IdempotencyError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IdempotencyError::EmptyKey);
        }

        Ok(Self(value))
    }
}

/// Resultado local almacenado para una llave de idempotencia.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordedResult {
    /// El primer intento fue aceptado por el modelo.
    Accepted,
    /// El primer intento fue rechazado por el modelo.
    Rejected,
}

/// Fallas de validación del registro local.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdempotencyError {
    /// Una llave vacía no puede correlacionar reintentos.
    EmptyKey,
}

/// Registro determinista de resultados sintéticos en memoria.
#[derive(Debug, Default)]
pub struct IdempotencyRegistry {
    records: HashMap<IdempotencyKey, RecordedResult>,
}

impl IdempotencyRegistry {
    /// Registra el primer resultado y conserva ese mismo valor en reintentos.
    pub fn record(&mut self, key: IdempotencyKey, proposed: RecordedResult) -> RecordedResult {
        *self.records.entry(key).or_insert(proposed)
    }

    /// Cuenta las intenciones distintas registradas en esta demostración.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Indica si el registro no contiene intentos.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{IdempotencyKey, IdempotencyRegistry, RecordedResult};

    #[test]
    fn un_reintento_conserva_el_primer_resultado() {
        let key = IdempotencyKey::new("retry-demo").expect("llave válida");
        let mut registry = IdempotencyRegistry::default();

        assert_eq!(
            registry.record(key.clone(), RecordedResult::Accepted),
            RecordedResult::Accepted
        );
        assert_eq!(
            registry.record(key, RecordedResult::Rejected),
            RecordedResult::Accepted
        );
        assert_eq!(registry.len(), 1);
    }
}
