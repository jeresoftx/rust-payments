//! Intercambio de adaptadores sin cambiar el contrato interno.

use rust_payments::provider::{
    AcceptingProvider, PaymentProvider, ProviderRequest, RejectingProvider,
};

fn main() {
    let request = ProviderRequest {
        reference: "provider-example-001",
    };
    println!("acepta: {:?}", AcceptingProvider.submit(request));
    println!("rechaza: {:?}", RejectingProvider.submit(request));
}
