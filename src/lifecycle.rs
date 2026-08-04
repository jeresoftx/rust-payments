//! Máquina de estados sintética para el recorrido de una transacción.

/// Estado observable de un flujo pedagógico, no de una operación financiera.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionState {
    Created,
    Authorized,
    Captured,
    Settled,
    Reversed,
    Refunded,
}

/// Acciones que pueden solicitarse sobre el modelo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    Authorize,
    Capture,
    Settle,
    Reverse,
    Refund,
}

/// Resultado de intentar una transición prohibida.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidTransition {
    pub from: TransactionState,
    pub requested: Transition,
}

impl TransactionState {
    /// Aplica únicamente transiciones declaradas por el modelo.
    pub const fn apply(self, transition: Transition) -> Result<Self, InvalidTransition> {
        match (self, transition) {
            (Self::Created, Transition::Authorize) => Ok(Self::Authorized),
            (Self::Authorized, Transition::Capture) => Ok(Self::Captured),
            (Self::Authorized, Transition::Reverse) => Ok(Self::Reversed),
            (Self::Captured, Transition::Settle) => Ok(Self::Settled),
            (Self::Settled, Transition::Refund) => Ok(Self::Refunded),
            _ => Err(InvalidTransition {
                from: self,
                requested: transition,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TransactionState, Transition};
    #[test]
    fn captura_requiere_autorizacion_sintetica() {
        assert!(
            TransactionState::Created
                .apply(Transition::Capture)
                .is_err()
        );
        assert_eq!(
            TransactionState::Created
                .apply(Transition::Authorize)
                .expect("transición válida"),
            TransactionState::Authorized
        );
    }
    #[test]
    fn reembolso_requiere_liquidacion_sintetica() {
        let settled = TransactionState::Captured
            .apply(Transition::Settle)
            .expect("transición válida");
        assert_eq!(
            settled
                .apply(Transition::Refund)
                .expect("transición válida"),
            TransactionState::Refunded
        );
    }
}
