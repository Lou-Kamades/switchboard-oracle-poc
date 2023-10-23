use std::collections::HashMap;
use pyth_sdk_solana::load_price_feed_from_account;

use serde::Deserialize;
pub use switchboard_solana::prelude::*;
use switchboard_utils::{
    handle_reqwest_err,
    protos::{JupiterSwapClient, JupiterSwapQuoteResponse, TokenInput},
};

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{solana_client::nonblocking::rpc_client::RpcClient, solana_sdk::pubkey};
use tokio::try_join;

pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const PYTH_USDC_ADDRESS: Pubkey = pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
pub const JUPITER_TOKEN: &str = "TOKEN";


pub async fn fetch_prices(
    rpc_client: &RpcClient,
    runner: &FunctionRunner,
    token_mints: Vec<&str>,
) -> Result<HashMap<String, JupiterTokenPrice>> {
    let price_future = try_join!(fetch_jupiter_prices(token_mints), fetch_usdc_price_from_pyth(&rpc_client, runner));

    if price_future.is_err() {
        runner.emit_error(23).await?;
    }

    let (mut jupiter_prices, usdc_price) = price_future?; 
    for (_, price) in jupiter_prices.iter_mut() {
        price.price *= usdc_price;
    }

    Ok(jupiter_prices)
}

/// Uses the V4 Jupiter price API. All prices are relative to USDC.
pub async fn fetch_jupiter_prices(
    token_mints: Vec<&str>,
) -> Result<HashMap<String, JupiterTokenPrice>> {
    let mint_string = token_mints.join("%2C");
    let url = format!("https://quote-api.jup.ag/v4/price?ids={}&vsToken=USDC&token={}", mint_string, JUPITER_TOKEN);
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
    Ok(response.data)
}

pub async fn fetch_usdc_price_from_pyth(
    rpc_client: &RpcClient,
    runner: &FunctionRunner,
) -> Result<f64> {
    let account_result = rpc_client.get_account(&PYTH_USDC_ADDRESS).await;
    if account_result.is_err() {
        runner.emit_error(21).await?;
    };
    let mut usdc_price_account = account_result.unwrap();

    let feed_result = load_price_feed_from_account(&PYTH_USDC_ADDRESS, &mut usdc_price_account);
    if feed_result.is_err() {
        runner.emit_error(22).await?;
    };
    let usdc_price_feed = feed_result.unwrap();
    let price = usdc_price_feed.get_price_unchecked();
    let usdc_price = (price.price as f64) * 10f64.powf(price.expo as f64); // TODO: safer math

    Ok(usdc_price)
}


pub async fn fetch_orca_prices(_runner: &FunctionRunner) -> Result<()> {
    unimplemented!("LpExchangeRateTask");
}



// pub async fn fetch_jupiter_quotes(
//     runner: &FunctionRunner,
//     token: &TokenInput,
// ) -> Result<Vec<JupiterSwapQuoteResponse>> {
//     let mint = token.address.clone();

//     let x = fetch_jupiter_prices(vec![&mint]).await?;
//     println!("{:?}, {:?}", token.address, x);
//     let token_price = x.get(&mint).unwrap().price;

//     // 500, 1000, 5000, 10000
//     let amounts: Vec<(String, String)> = [
//         500_000_000f64,
//         1000_000_000f64,
//         5000_000_000f64,
//         10000_000_000f64,
//     ]
//     .into_iter()
//     .map(|a| (a.to_string(), ((token_price * a) as u64).to_string()))
//     .collect();
//     fetch_jupiter_quotes_inner(runner, token, amounts).await
// }

// async fn fetch_jupiter_quotes_inner(
//     runner: &FunctionRunner,
//     token: &TokenInput,
//     amounts: Vec<(String, String)>,
// ) -> Result<Vec<JupiterSwapQuoteResponse>> {
//     let x = JupiterSwapClient::new(Some(JUPITER_TOKEN.to_string()));

//     let usdc_token = TokenInput {
//         address: USDC_MINT.to_string(),
//         decimals: 6,
//     };

//     let mut quote_requests = vec![];

//     for (amount, pair_amount) in amounts.iter() {
//         quote_requests.push(x.get_quote(&usdc_token, token, amount, None));
//         quote_requests.push(x.get_quote(&usdc_token, token, pair_amount, None));
//     }

//     let mut results = vec![];
//     let quote_results = join_all(quote_requests).await;

//     for q in quote_results.into_iter() {
//         if q.is_err() {
//             runner.emit_error(33).await?;
//         } else {
//             results.push(q.unwrap());
//         };
//     }

//     Ok(results)
// }


// pub fn estimate_price_from_quote(quote: &JupiterSwapQuoteResponse, token: &TokenInput, usdc_price: f64) -> f64 {
//     if &quote.input_mint == USDC_MINT {
//         let in_amount = normalize_and_convert_to_f64(quote.in_amount, 6) * usdc_price;
//         let out_amount = normalize_and_convert_to_f64(quote.out_amount, token.decimals);
//         in_amount / out_amount
//     } else {
//         let in_amount = normalize_and_convert_to_f64(quote.in_amount, token.decimals);
//         let out_amount = normalize_and_convert_to_f64(quote.out_amount, 6) * usdc_price;
//         in_amount / out_amount
//     }
// }

// pub fn calculate_avg_price_and_std_dev(
//     quotes: &Vec<JupiterSwapQuoteResponse>,
//     token: &TokenInput,
//     usdc_price: f64
// ) -> (f64, f64) {
//     let prices: Vec<f64> = quotes
//         .iter()
//         .map(|q| estimate_price_from_quote(&q, token, usdc_price))
//         .collect();

//     let mean = prices.iter().sum::<f64>() / (prices.len() as f64);
//     let variance: f64 = prices
//         .iter()
//         .map(|p| {
//             let diff = p - mean;
//             diff * diff
//         })
//         .sum();

//     let std_dev = variance.sqrt();

//     (mean, std_dev)
// }

// fn normalize_and_convert_to_f64(value: u64, decimal_places: u32) -> f64 {
//     let divisor = 10u64.pow(decimal_places) as f64;
//     (value as f64) / divisor
// }

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
