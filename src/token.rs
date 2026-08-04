//! Referencias sintéticas con alcance limitado.

/// Propósito permitido para una referencia del ejemplo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenPurpose {
    /// La referencia puede usarse en una intención de cobro sintética.
    Charge,
    /// La referencia puede usarse en una devolución sintética.
    Refund,
}

/// Referencia local que no contiene un secreto ni un dato financiero.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntheticToken {
    reference: String,
    purpose: TokenPurpose,
    expires_at_tick: u64,
}
impl SyntheticToken {
    /// Crea una referencia local limitada a un propósito y vencimiento lógico.
    #[must_use]
    pub fn new(reference: impl Into<String>, purpose: TokenPurpose, expires_at_tick: u64) -> Self {
        Self {
            reference: reference.into(),
            purpose,
            expires_at_tick,
        }
    }

    /// Indica si la referencia sigue vigente para el propósito solicitado.
    #[must_use]
    pub fn is_valid_at(&self, tick: u64, purpose: TokenPurpose) -> bool {
        tick <= self.expires_at_tick && self.purpose == purpose && !self.reference.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{SyntheticToken, TokenPurpose};
    #[test]
    fn el_token_solo_vale_para_su_proposito_y_vigencia() {
        let token = SyntheticToken::new("token-demo", TokenPurpose::Charge, 3);

        assert!(token.is_valid_at(3, TokenPurpose::Charge));
        assert!(!token.is_valid_at(4, TokenPurpose::Charge));
        assert!(!token.is_valid_at(1, TokenPurpose::Refund));
    }
}
