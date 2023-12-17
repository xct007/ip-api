use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct R {
    pub status: bool,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub result: Option<Rr>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Rr {
    pub ip: String,
}
