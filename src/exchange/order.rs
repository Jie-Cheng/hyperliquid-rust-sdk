use crate::{helpers::float_to_string_for_hashing, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Limit {
    pub tif: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub trigger_px: String,
    pub is_market: bool,
    pub tpsl: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Limit(Limit),
    Trigger(Trigger),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    #[serde(rename = "a", alias = "asset")]
    pub asset: u32,
    #[serde(rename = "b", alias = "isBuy")]
    pub is_buy: bool,
    #[serde(rename = "p", alias = "limitPx")]
    pub limit_px: String,
    #[serde(rename = "s", alias = "sz")]
    pub sz: String,
    #[serde(rename = "r", alias = "reduceOnly", default)]
    pub reduce_only: bool,
    #[serde(rename = "t", alias = "orderType")]
    pub order_type: Order,
    #[serde(rename = "c", alias = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

pub struct ClientLimit {
    pub tif: String,
}

pub struct ClientTrigger {
    pub trigger_px: f64,
    pub is_market: bool,
    pub tpsl: String,
}

pub enum ClientOrder {
    Limit(ClientLimit),
    Trigger(ClientTrigger),
}
pub struct ClientOrderRequest {
    pub asset_id: u32,
    pub is_buy: bool,
    pub reduce_only: bool,
    pub limit_px: f64,
    pub sz: f64,
    pub cloid: String,
    pub order_type: ClientOrder,
}

impl ClientOrderRequest {
    pub(crate) fn convert(self) -> Result<OrderRequest> {
        let order_type = match self.order_type {
            ClientOrder::Limit(limit) => Order::Limit(Limit { tif: limit.tif }),
            ClientOrder::Trigger(trigger) => Order::Trigger(Trigger {
                trigger_px: float_to_string_for_hashing(trigger.trigger_px),
                is_market: trigger.is_market,
                tpsl: trigger.tpsl,
            }),
        };

        Ok(OrderRequest {
            asset: self.asset_id,
            is_buy: self.is_buy,
            reduce_only: self.reduce_only,
            limit_px: float_to_string_for_hashing(self.limit_px),
            sz: float_to_string_for_hashing(self.sz),
            cloid: Some(self.cloid),
            order_type: order_type,
        })
    }
}
