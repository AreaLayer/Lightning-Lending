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

