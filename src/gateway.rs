//! Contratos sintéticos del modelo de una pasarela.
//!
//! Estos tipos hacen visible quién participa y qué resultado devuelve una
//! integración pedagógica. No representan tarjetas, cuentas, dinero ni una
//! autorización de producción.

/// Papel de una parte dentro del recorrido conceptual de un pago.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Actor {
    /// Sistema que inicia una intención sintética.
    Merchant,
    /// Frontera que traduce la intención en un resultado observable.
    Gateway,
    /// Parte conceptual que representa al adquirente.
    Acquirer,
    /// Parte conceptual que representa una red de pago.
    Network,
    /// Parte conceptual que representa al emisor.
    Issuer,
}

/// Identificador local de una intención sintética.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntentId(String);

impl IntentId {
    /// Crea un identificador no vacío sin significado fuera del ejemplo.
    pub fn new(value: impl Into<String>) -> Result<Self, GatewayError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(GatewayError::EmptyIntentId);
        }

        Ok(Self(value))
    }

    /// Expone la referencia local para imprimirla en ejemplos y auditorías.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Estado observable de la intención dentro de este modelo pedagógico.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntentStatus {
    /// La intención llegó a la frontera de integración.
    Received,
    /// El modelo la aceptó como una solicitud sintácticamente válida.
    Accepted,
    /// El modelo la rechazó antes de representar cualquier operación externa.
    Rejected,
}

/// Intención que nunca contiene datos financieros ni personales.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntheticIntent {
    id: IntentId,
    status: IntentStatus,
}

impl SyntheticIntent {
    /// Registra una intención local que inicia en estado `Received`.
    #[must_use]
    pub fn received(id: IntentId) -> Self {
        Self {
            id,
            status: IntentStatus::Received,
        }
    }

    /// Devuelve la referencia local de la intención.
    #[must_use]
    pub fn id(&self) -> &IntentId {
        &self.id
    }

    /// Devuelve el estado que el modelo permite observar.
    #[must_use]
    pub const fn status(&self) -> IntentStatus {
        self.status
    }
}

/// Error de validación del modelo de pasarela sintético.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayError {
    /// Las referencias vacías no permiten correlacionar el ejemplo.
    EmptyIntentId,
}

/// Evalúa una intención sin comunicarse con ningún sistema externo.
///
/// La aceptación significa únicamente que el identificador local existe; no
/// equivale a autorización, captura, liquidación ni cobro.
#[must_use]
pub fn evaluate(intent: SyntheticIntent) -> SyntheticIntent {
    SyntheticIntent {
        id: intent.id,
        status: IntentStatus::Accepted,
    }
}

#[cfg(test)]
mod tests {
    use super::{GatewayError, IntentId, IntentStatus, SyntheticIntent, evaluate};

    #[test]
    fn una_intencion_recibida_se_vuelve_aceptada_solo_en_el_modelo() {
        let id = IntentId::new("intent-demo-001").expect("referencia sintética válida");
        let result = evaluate(SyntheticIntent::received(id));

        assert_eq!(result.status(), IntentStatus::Accepted);
        assert_eq!(result.id().as_str(), "intent-demo-001");
    }

    #[test]
    fn una_referencia_vacia_no_es_correlacionable() {
        assert_eq!(IntentId::new("  "), Err(GatewayError::EmptyIntentId));
    }
}
