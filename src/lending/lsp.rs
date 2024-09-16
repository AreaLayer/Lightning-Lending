use lightning_liquidity::events::Event;
use lightning_liquidity::events::lsp::{LSPS0ClientEvent, LSPS2ClientEvent, LSPS2ServiceEvent};
use lightning_liquidity::LSPS0Client;

pub enum Event {
    LSPS0Client(LSPS0ClientEvent),
    LSPS2Client(LSPS2ClientEvent),
    LSPS2Service(LSPS2ServiceEvent),
}

impl From<LSPS0ClientEvent> for Event {
    fn from(event: LSPS0ClientEvent) -> Self {
        Event::LSPS0Client(event)
    }
}

impl From<LSPS2ClientEvent> for Event {
    fn from(event: LSPS2ClientEvent) -> Self {
        Event::LSPS2Client(event)
    }
}

impl From<LSPS2ServiceEvent> for Event {
    fn from(event: LSPS2ServiceEvent) -> Self {
        Event::LSPS2Service(event)
    }
}

fn lightning_liquidity_service_test() {
    let mut service = LSPS2Service::new(
        LSPS0Client::new(
            event_stream()
        )
    );
    }

fn lightning_liquidity_service_test() {
    let mut service = LSPS2ServiceEvent::new(
        LSPS2ServiceEvent::new(
            event_stream()
        )
    );
    }

fn lightning_liquidity_service_test() {
    let mut service = LSPS2ServiceEvent::new(
        LSPS2ServiceEvent::new(
            event_stream()
        )
    );
    }