use std::collections::HashMap;

use futures::future::join_all;
use pyth_sdk::Price;
use pyth_sdk_solana::load_price_feed_from_account;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
pub use switchboard_solana::prelude::*;
use switchboard_utils::{
    handle_reqwest_err,
    protos::{JupiterSwapClient, JupiterSwapQuoteResponse, TokenInput},
};

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{solana_client::nonblocking::rpc_client::RpcClient, solana_sdk::pubkey};
use tokio::{join, try_join};

pub async fn fetch_jupiter_quotes(
    runner: &FunctionRunner,
    token: TokenInput,
) -> Result<Vec<JupiterSwapQuoteResponse>> {
    let mint = token.address.clone();

    let x = fetch_jupiter_prices(vec![mint.clone()]).await?;
    let token_price = x.get(&mint).unwrap().price;

    // 500, 1000, 5000, 10000
    let amounts: Vec<(String, String)> = [
        500_000_000f64,
        1000_000_000f64,
        5000_000_000f64,
        10000_000_000f64
    ]
    .into_iter()
    .map(|a| (a.to_string(), (token_price * a).to_string()))
    .collect();
    fetch_jupiter_quotes_inner(runner, token, amounts).await
}

async fn fetch_jupiter_quotes_inner(
    runner: &FunctionRunner,
    token: TokenInput,
    amounts: Vec<(String, String)>,
) -> Result<Vec<JupiterSwapQuoteResponse>> {
    let x = JupiterSwapClient::new(Some("TOKEN".to_string()));

    let usdc_token = TokenInput {
        address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        decimals: 6,
    };

    let mut quote_requests = vec![];

    for (amount, pair_amount) in amounts.iter() {
        quote_requests.push(x.get_quote(&usdc_token, &token, amount, None));
        quote_requests.push(x.get_quote(&usdc_token, &token, pair_amount, None));
    }

    let mut results = vec![];
    let quote_results = join_all(quote_requests).await;

    for q in quote_results.into_iter() {
        if q.is_err() {
            runner.emit_error(33).await?;
        } else {
            results.push(q.unwrap());
        };
    }

    Ok(results)
}

pub async fn fetch_orca_prices(_runner: &FunctionRunner) -> Result<()> {
    unimplemented!("LpExchangeRateTask");
}

pub async fn fetch_usd_price_from_pyth(
    rpc_client: &RpcClient,
    runner: &FunctionRunner,
) -> Result<Price> {
    let usdc_price_key: Pubkey = pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");

    let account_result = rpc_client.get_account(&usdc_price_key).await;
    if account_result.is_err() {
        runner.emit_error(21).await?;
    };
    let mut usdc_price_account = account_result.unwrap();

    let feed_result = load_price_feed_from_account(&usdc_price_key, &mut usdc_price_account);
    if feed_result.is_err() {
        runner.emit_error(22).await?;
    };
    let usdc_price_feed = feed_result.unwrap();
    let price = usdc_price_feed.get_price_unchecked();
    Ok(price)
}

/// Uses the V4 Jupiter price API to fetch an approximate price that is used to derive size for the V6 quotes
pub async fn fetch_jupiter_prices(
    token_mints: Vec<String>,
) -> Result<HashMap<String, JupiterTokenPrice>> {
    let mint_string = token_mints.join("%2C");
    let url = format!("https://quote-api.jup.ag/v4/price?ids={}", mint_string);
    let response = reqwest::get(url)
        .await
        .map_err(handle_reqwest_err)?
        .error_for_status()
        .map_err(handle_reqwest_err)?;

    if response.status() != 200 {
        return Err(Box::new(SbError::CustomMessage(format!(
            "Jupiter Price API returned status code {}",
            response.status()
        ))));
    }

    // Get the response text as a string
    let text = response.text().await.map_err(handle_reqwest_err)?;

    let response: JupiterPriceResponse = serde_json::from_str(&text).unwrap();
    println!("{:?}", response);

    Ok(response.data)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterPriceResponse {
    pub data: HashMap<String, JupiterTokenPrice>,

    pub time_taken: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterTokenPrice {
    pub id: String,

    pub mint_symbol: String,

    pub vs_token: String,

    pub vs_token_symbol: String,

    pub price: f64,
}
