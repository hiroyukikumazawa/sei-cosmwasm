use crate::sei_types::Order;
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// implement custom query
impl CustomMsg for SeiMsg {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<SeiMsg> for CosmosMsg<SeiMsg> {
    fn from(original: SeiMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiMsg {
    PlaceOrders {
        orders: Vec<Order>,
        funds: Vec<Coin>,
        contract_address: Addr,
    },
    CancelOrders {
        order_ids: Vec<u64>,
        contract_address: Addr,
    },
    CreateDenom {
        subdenom: String,
    },
    Mint {
        amount: Coin,
    },
    Burn {
        amount: Coin,
    },
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
}
