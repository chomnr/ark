use serde::{Serialize, Deserialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    permission_id: String,
    permission_name: String,
    permission_key: String
}