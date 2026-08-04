//! Recorrido mínimo de una intención sintética.

use rust_payments::gateway::{IntentId, SyntheticIntent, evaluate};

fn main() {
    let id = IntentId::new("intent-example-001").expect("la referencia del ejemplo no está vacía");
    let accepted = evaluate(SyntheticIntent::received(id));

    println!(
        "{} quedó en estado {:?}",
        accepted.id().as_str(),
        accepted.status()
    );
}
