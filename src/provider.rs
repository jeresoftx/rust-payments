//! Puerto interno y adaptadores totalmente simulados.

/// Solicitud mínima que el dominio entrega a un proveedor simulado.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderRequest {
    /// Referencia local, nunca un identificador financiero real.
    pub reference: &'static str,
}

/// Resultado normalizado que devuelve el puerto.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderOutcome {
    /// El adaptador simuló una aceptación.
    Accepted,
    /// El adaptador simuló un rechazo.
    Rejected,
}

/// Contrato estable que puede implementar un proveedor simulado.
pub trait PaymentProvider {
    /// Procesa una solicitud local sin red ni credenciales.
    fn submit(&self, request: ProviderRequest) -> ProviderOutcome;
}

/// Adaptador determinista que acepta referencias no vacías.
#[derive(Clone, Copy, Debug, Default)]
pub struct AcceptingProvider;

impl PaymentProvider for AcceptingProvider {
    fn submit(&self, request: ProviderRequest) -> ProviderOutcome {
        if request.reference.is_empty() {
            ProviderOutcome::Rejected
        } else {
            ProviderOutcome::Accepted
        }
    }
}

/// Adaptador determinista que permite ensayar una respuesta negativa.
#[derive(Clone, Copy, Debug, Default)]
pub struct RejectingProvider;

impl PaymentProvider for RejectingProvider {
    fn submit(&self, _request: ProviderRequest) -> ProviderOutcome {
        ProviderOutcome::Rejected
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AcceptingProvider, PaymentProvider, ProviderOutcome, ProviderRequest, RejectingProvider,
    };

    #[test]
    fn el_puerto_normaliza_dos_adaptadores_simulados() {
        let request = ProviderRequest {
            reference: "provider-demo",
        };

        assert_eq!(AcceptingProvider.submit(request), ProviderOutcome::Accepted);
        assert_eq!(RejectingProvider.submit(request), ProviderOutcome::Rejected);
    }
}
