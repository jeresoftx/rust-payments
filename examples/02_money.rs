//! Suma y división explícita de dinero sintético.

use rust_payments::money::{Currency, Money, Rounding};

fn main() {
    let total = Money::new(1_000, Currency::Mxn);
    let share = total
        .split(3, Rounding::AwayFromZero)
        .expect("la regla de redondeo permite el residuo");

    println!(
        "cada parte conserva {} unidades menores",
        share.minor_units()
    );
}
