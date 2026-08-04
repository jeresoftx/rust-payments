//! Recorrido válido de un ciclo de vida sintético.
use rust_payments::lifecycle::{TransactionState, Transition};

fn main() {
    let authorized = TransactionState::Created
        .apply(Transition::Authorize)
        .expect("autorización sintética");
    let captured = authorized
        .apply(Transition::Capture)
        .expect("captura sintética");
    let settled = captured
        .apply(Transition::Settle)
        .expect("liquidación sintética");
    println!("estado final: {settled:?}");
}
