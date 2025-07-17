use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use ethers::providers::{Provider, Http};
use crate::blockchain::get_eth_balance;

#[derive(Deserialize)]
pub struct Query {
    address: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    address: String,
    balance_eth: String,
}

pub fn routes() -> Router {
    Router::new().route("/balance", get(get_balance_handler))
}

async fn get_balance_handler(
    axum::extract::Query(query): axum::extract::Query<Query>
) -> Json<BalanceResponse> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());

    match get_eth_balance(&query.address, provider.clone()).await {
        Ok(balance) => {
            let eth = ethers::utils::format_units(balance, 18).unwrap_or_else(|_| "0".to_string());
            Json(BalanceResponse {
                address: query.address,
                balance_eth: eth,
            })
        }
        Err(err) => Json(BalanceResponse {
            address: query.address,
            balance_eth: format!("error: {}", err),
        }),
    }
}
