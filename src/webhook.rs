//! Receptor en memoria para eventos sintéticos fuera de orden.

use std::collections::HashSet;

/// Evento local con un identificador y secuencia sintéticos.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookEvent {
    /// Identificador único dentro de la demostración.
    pub id: String,
    /// Secuencia que permite detectar mensajes tardíos.
    pub sequence: u64,
}

/// Resultado de intentar aplicar un evento.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebhookDisposition {
    /// El evento avanzó el estado local.
    Applied,
    /// El identificador ya había sido procesado.
    Duplicate,
    /// El evento llegó después de una secuencia más reciente.
    Stale,
}

/// Receptor que conserva la última secuencia y eventos vistos en memoria.
#[derive(Debug, Default)]
pub struct WebhookReceiver {
    seen: HashSet<String>,
    latest_sequence: Option<u64>,
}

impl WebhookReceiver {
    /// Deduplica y rechaza eventos tardíos sin validar una firma real.
    pub fn receive(&mut self, event: WebhookEvent) -> WebhookDisposition {
        if !self.seen.insert(event.id) {
            return WebhookDisposition::Duplicate;
        }
        if self
            .latest_sequence
            .is_some_and(|latest| event.sequence < latest)
        {
            return WebhookDisposition::Stale;
        }
        self.latest_sequence = Some(event.sequence);
        WebhookDisposition::Applied
    }
}

#[cfg(test)]
mod tests {
    use super::{WebhookDisposition, WebhookEvent, WebhookReceiver};

    #[test]
    fn deduplica_y_rechaza_eventos_tardios() {
        let mut receiver = WebhookReceiver::default();
        let current = WebhookEvent {
            id: "evt-2".into(),
            sequence: 2,
        };

        assert_eq!(
            receiver.receive(current.clone()),
            WebhookDisposition::Applied
        );
        assert_eq!(receiver.receive(current), WebhookDisposition::Duplicate);
        assert_eq!(
            receiver.receive(WebhookEvent {
                id: "evt-1".into(),
                sequence: 1
            }),
            WebhookDisposition::Stale
        );
    }
}
