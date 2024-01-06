pub mod model;
pub mod send;

#[derive(Clone)]
pub enum BridgeType {
    Role
}

#[derive(Clone)]
pub enum BridgePriority {
    HIGH,
    MEDIUM,
    LOW
}