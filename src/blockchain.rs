use ethers::prelude::*;
use std::sync::Arc;

pub async fn get_eth_balance(address: &str, provider: Arc<Provider<Http>>) -> Result<U256, String> {

    let addr = address.parse::<Address>().map_err(|e| e.to_string())?;

    provider.get_balance(addr, None).await.map_err(|e| e.to_string())
}
