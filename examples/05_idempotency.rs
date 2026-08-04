//! Un reintento conserva el resultado local inicial.

use rust_payments::idempotency::{IdempotencyKey, IdempotencyRegistry, RecordedResult};

fn main() {
    let key = IdempotencyKey::new("retry-example-001").expect("llave no vacía");
    let mut registry = IdempotencyRegistry::default();

    let first = registry.record(key.clone(), RecordedResult::Accepted);
    let retry = registry.record(key, RecordedResult::Rejected);
    println!("primer resultado: {first:?}; reintento: {retry:?}");
}
