//! Decisiones explicables de riesgo sintético.

use rust_payments::risk::{RiskSignals, decide};

fn main() {
    let signals = RiskSignals {
        unknown_device: true,
        high_frequency: true,
        ..RiskSignals::default()
    };
    println!("decisión: {:?}", decide(signals));
}
