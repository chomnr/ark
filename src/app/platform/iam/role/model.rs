use serde::{Serialize, Deserialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    role_id: String,
    role_name: String
}