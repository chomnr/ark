use super::{model::BridgeEvent, error::BridgeEventResult};

pub struct BridgeSender {
    event: BridgeEvent
}

impl BridgeSender {
    pub fn send(event: BridgeEvent) -> BridgeEventResult<bool>  {
        // execute
        todo!()
    }
}