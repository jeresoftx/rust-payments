//! Comparación de evidencia sintética para un ejercicio de conciliación.

/// Resultado normalizado que puede aparecer en dos fuentes de evidencia.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceStatus {
    /// La evidencia indica aceptación sintética.
    Accepted,
    /// La evidencia indica rechazo sintético.
    Rejected,
}

/// Decisión visible al comparar dos evidencias locales.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconciliationResult {
    /// Ambas evidencias expresan el mismo resultado.
    Match,
    /// Las evidencias difieren y requieren investigación humana posterior.
    Mismatch,
}

/// Compara evidencia interna y de proveedor simulada sin reparar diferencias.
#[must_use]
pub fn reconcile(internal: EvidenceStatus, provider: EvidenceStatus) -> ReconciliationResult {
    if internal == provider {
        ReconciliationResult::Match
    } else {
        ReconciliationResult::Mismatch
    }
}

#[cfg(test)]
mod tests {
    use super::{EvidenceStatus, ReconciliationResult, reconcile};

    #[test]
    fn una_discrepancia_permanece_visible() {
        assert_eq!(
            reconcile(EvidenceStatus::Accepted, EvidenceStatus::Rejected),
            ReconciliationResult::Mismatch
        );
        assert_eq!(
            reconcile(EvidenceStatus::Accepted, EvidenceStatus::Accepted),
            ReconciliationResult::Match
        );
    }
}
