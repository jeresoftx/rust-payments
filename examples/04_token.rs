//! Uso acotado de una referencia sintética.

use rust_payments::token::{SyntheticToken, TokenPurpose};

fn main() {
    let token = SyntheticToken::new("token-example-001", TokenPurpose::Charge, 10);
    println!(
        "vigente para cobro: {}",
        token.is_valid_at(8, TokenPurpose::Charge)
    );
    println!(
        "vigente para devolución: {}",
        token.is_valid_at(8, TokenPurpose::Refund)
    );
}
