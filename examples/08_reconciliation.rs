//! Una discrepancia sintética se conserva como evidencia.

use rust_payments::reconciliation::{EvidenceStatus, reconcile};

fn main() {
    let result = reconcile(EvidenceStatus::Accepted, EvidenceStatus::Rejected);
    println!("resultado de conciliación: {result:?}");
}
