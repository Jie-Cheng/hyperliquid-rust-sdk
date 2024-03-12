use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct ClientCancelRequest {
    pub asset: String,
    pub oid: u64,
}

pub struct ClientCancelRequestCloid {
    pub asset: String,
    pub cloid: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelRequestCloid {
    pub asset: u32,
    pub cloid: String,
}