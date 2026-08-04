//! Eventos sintéticos duplicados y fuera de orden.

use rust_payments::webhook::{WebhookEvent, WebhookReceiver};

fn main() {
    let mut receiver = WebhookReceiver::default();
    let event = WebhookEvent {
        id: "event-example-002".into(),
        sequence: 2,
    };
    println!("primer evento: {:?}", receiver.receive(event.clone()));
    println!("duplicado: {:?}", receiver.receive(event));
}
