use super::model::BridgeEvent;

pub struct BridgeSender {
    event: BridgeEvent
}

impl BridgeSender {
    pub fn new(event: BridgeEvent) -> Self {
        Self {
            event
        }
    }
}