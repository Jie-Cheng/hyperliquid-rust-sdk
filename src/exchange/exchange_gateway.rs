use crate::{
    exchange::{
        actions::BulkOrder, cancel::CancelRequestCloid, ClientLimit, ClientOrder,
        ClientOrderRequest,
    },
    prelude::*,
    signature::sign_l1_action,
    BulkCancelCloid, Error,
};
use ethers::{
    signers::LocalWallet,
    types::{Signature, H160, H256},
};
use itertools::izip;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum GatewayActions {
    Order(BulkOrder),
    CancelByCloid(BulkCancelCloid),
}

impl GatewayActions {
    fn hash(&self, timestamp: u64, vault_address: Option<H160>) -> Result<H256> {
        let mut bytes =
            rmp_serde::to_vec_named(self).map_err(|e| Error::RmpParse(e.to_string()))?;
        bytes.extend(timestamp.to_be_bytes());
        if let Some(vault_address) = vault_address {
            bytes.push(1);
            bytes.extend(vault_address.to_fixed_bytes());
        } else {
            bytes.push(0);
        }
        Ok(H256(ethers::utils::keccak256(bytes)))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExchangePayload {
    action: serde_json::Value,
    signature: Signature,
    nonce: u64,
    vault_address: Option<H160>,
}

pub fn bulk_cancel(
    wallet: &LocalWallet,
    mainnet: bool,
    asset_id: u32,
    cloid: Vec<String>, // eg {"0x1c0f0be5594940158311413cb05b34cc"}
    nonce: u64,
    vault: Option<H160>,
) -> Result<String> {
    let mut transformed_cancels = Vec::new();

    for c in &cloid {
        let cancel = CancelRequestCloid {
            asset: asset_id,
            cloid: c.to_string(),
        };
        transformed_cancels.push(cancel);
    }

    let action = GatewayActions::CancelByCloid(BulkCancelCloid {
        cancels: transformed_cancels,
    });
    let connection_id = action.hash(nonce, vault)?;
    let signature = sign_l1_action(&wallet, connection_id, mainnet)?;
    let action = serde_json::to_value(&action).map_err(|e| Error::JsonParse(e.to_string()))?;

    let exchange_payload = ExchangePayload {
        action,
        signature,
        nonce,
        vault_address: vault,
    };

    let res =
        serde_json::to_string(&exchange_payload).map_err(|e| Error::JsonParse(e.to_string()))?;

    Ok(res.to_string())
}

pub fn bulk_order(
    wallet: &LocalWallet,
    mainnet: bool,
    asset_id: u32,
    cloid: Vec<String>,
    is_maker: Vec<bool>,
    is_buy: Vec<bool>,
    limit_px: Vec<f64>,
    sz: Vec<f64>,
    nonce: u64,
    vault: Option<H160>,
) -> Result<String> {
    let mut transformed_orders = Vec::new();

    for (c, m, b, p, s) in izip!(&cloid, &is_maker, &is_buy, &limit_px, &sz) {
        let order_type = if *m {
            ClientOrder::Limit(ClientLimit {
                tif: "Alo".to_string(),
            })
        } else {
            ClientOrder::Limit(ClientLimit {
                tif: "Ioc".to_string(),
            })
        };
        let order = ClientOrderRequest {
            asset_id: asset_id,
            is_buy: *b,
            reduce_only: false,
            limit_px: *p,
            sz: *s,
            cloid: c.to_string(),
            order_type: order_type,
        };
        transformed_orders.push(order.convert()?);
    }

    let action = GatewayActions::Order(BulkOrder {
        orders: transformed_orders,
        grouping: "na".to_string(),
    });
    let connection_id = action.hash(nonce, vault)?;
    let signature = sign_l1_action(&wallet, connection_id, mainnet)?;

    let action = serde_json::to_value(&action).map_err(|e| Error::JsonParse(e.to_string()))?;

    let exchange_payload = ExchangePayload {
        action,
        signature,
        nonce,
        vault_address: vault,
    };

    let res =
        serde_json::to_string(&exchange_payload).map_err(|e| Error::JsonParse(e.to_string()))?;

    Ok(res.to_string())
}
