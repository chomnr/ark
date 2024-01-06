use nanoid::nanoid;

use super::{BridgePriority, BridgeType, send::BridgeSender};

pub struct BridgeEvent {
    bridge_id: String,
    bridge_type: Option<BridgeType>,
    bridge_payload: Option<String>,
    priority: BridgePriority,
}

impl BridgeEvent {
    pub fn new(
        bridge_id: String,
        bridge_type: Option<BridgeType>,
        bridge_payload: Option<String>,
        priority: BridgePriority,
    ) -> Self {
        BridgeEvent {
            bridge_id,
            bridge_type,
            bridge_payload,
            priority,
        }
    }

    pub fn builder() -> BridgeEventBuilder {
        BridgeEventBuilder::default()
    }

    pub fn send(self) -> BridgeSender {
        BridgeSender::new(self)
    }
}

pub struct BridgeEventBuilder {
    bridge_id: String,
    bridge_type: Option<BridgeType>,
    bridge_payload: Option<String>,
    priority: BridgePriority,
}

impl Default for BridgeEventBuilder {
    fn default() -> Self {
        Self {
            bridge_id: nanoid!(),
            bridge_type: None,
            bridge_payload: None,
            priority: BridgePriority::LOW,
        }
    }
}

impl BridgeEventBuilder {
    pub fn new() -> Self {
        BridgeEventBuilder::default()
    }

    pub fn bridge_type(&mut self, bridge_type: BridgeType) -> &mut Self {
        self.bridge_type = Some(bridge_type);
        self
    }

    pub fn bridge_payload(&mut self, payload: String) -> &mut Self {
        self.bridge_payload = Some(payload);
        self
    }

    pub fn priority(&mut self, priority: BridgePriority) -> &mut Self {
        self.priority = priority;
        self
    }

    pub fn build(&self) -> BridgeEvent {
        BridgeEvent::new(
            self.bridge_id.clone(),
            self.bridge_type.clone(),
            self.bridge_payload.clone(),
            self.priority.clone(),
        )
    }
}
