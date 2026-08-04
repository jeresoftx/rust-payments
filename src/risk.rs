//! Política de riesgo con señales completamente sintéticas.

/// Señales mínimas sin perfil, identidad ni dato personal.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RiskSignals {
    /// La solicitud requiere un desafío sintético adicional.
    pub challenge_requested: bool,
    /// El dispositivo de ejemplo no es reconocido.
    pub unknown_device: bool,
    /// El intento excede la frecuencia permitida por la demostración.
    pub high_frequency: bool,
}

/// Decisión explicable de la política.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskDecision {
    /// La demostración permite continuar.
    Allow,
    /// La demostración solicita un paso adicional.
    Challenge,
    /// La demostración rechaza por señales acumuladas.
    Reject,
}

/// Decide con reglas visibles, no con un modelo antifraude.
#[must_use]
pub const fn decide(signals: RiskSignals) -> RiskDecision {
    if signals.unknown_device && signals.high_frequency {
        RiskDecision::Reject
    } else if signals.challenge_requested || signals.unknown_device {
        RiskDecision::Challenge
    } else {
        RiskDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::{RiskDecision, RiskSignals, decide};
    #[test]
    fn las_senales_acumuladas_rechazan_la_demostracion() {
        assert_eq!(decide(RiskSignals::default()), RiskDecision::Allow);
        assert_eq!(
            decide(RiskSignals {
                unknown_device: true,
                ..RiskSignals::default()
            }),
            RiskDecision::Challenge
        );
        assert_eq!(
            decide(RiskSignals {
                unknown_device: true,
                high_frequency: true,
                ..RiskSignals::default()
            }),
            RiskDecision::Reject
        );
    }
}
