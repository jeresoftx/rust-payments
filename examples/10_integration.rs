//! El recorrido integrador completo del curso.

use rust_payments::integration::run_synthetic_flow;

fn main() {
    let report = run_synthetic_flow();
    println!("reporte sintético: {report:#?}");
}
