//! Recorrido integrador enteramente sintético y determinista.

use crate::idempotency::{IdempotencyKey, IdempotencyRegistry, RecordedResult};
use crate::provider::{AcceptingProvider, PaymentProvider, ProviderOutcome, ProviderRequest};
use crate::reconciliation::{EvidenceStatus, ReconciliationResult, reconcile};
use crate::webhook::{WebhookDisposition, WebhookEvent, WebhookReceiver};

/// Resumen auditable del flujo sintético del curso.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegrationReport {
    /// Resultado del adaptador simulado.
    pub provider: ProviderOutcome,
    /// Resultado del primer registro idempotente.
    pub idempotency: RecordedResult,
    /// Resultado del evento local.
    pub webhook: WebhookDisposition,
    /// Resultado de comparar evidencia sintética.
    pub reconciliation: ReconciliationResult,
}

/// Ejecuta el recorrido pedagógico sin red, dinero ni datos financieros.
pub fn run_synthetic_flow() -> IntegrationReport {
    let key = IdempotencyKey::new("integration-demo").expect("referencia estática no vacía");
    let mut registry = IdempotencyRegistry::default();
    let idempotency = registry.record(key, RecordedResult::Accepted);

    let provider = AcceptingProvider.submit(ProviderRequest {
        reference: "integration-demo",
    });
    let mut receiver = WebhookReceiver::default();
    let webhook = receiver.receive(WebhookEvent {
        id: "event-integration".into(),
        sequence: 1,
    });
    let reconciliation = reconcile(EvidenceStatus::Accepted, EvidenceStatus::Accepted);

    IntegrationReport {
        provider,
        idempotency,
        webhook,
        reconciliation,
    }
}

#[cfg(test)]
mod tests {
    use super::run_synthetic_flow;
    use crate::idempotency::RecordedResult;
    use crate::provider::ProviderOutcome;
    use crate::reconciliation::ReconciliationResult;
    use crate::webhook::WebhookDisposition;

    #[test]
    fn el_flujo_integrador_deja_evidencia_determinista() {
        let report = run_synthetic_flow();

        assert_eq!(report.provider, ProviderOutcome::Accepted);
        assert_eq!(report.idempotency, RecordedResult::Accepted);
        assert_eq!(report.webhook, WebhookDisposition::Applied);
        assert_eq!(report.reconciliation, ReconciliationResult::Match);
    }
}
