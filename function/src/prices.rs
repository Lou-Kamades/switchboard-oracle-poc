use anyhow::anyhow;
use pyth_sdk::Price;
use pyth_sdk_solana::load_price_feed_from_account;
use solana_account_decoder::UiAccountEncoding;
pub use switchboard_solana::prelude::*;
use switchboard_utils::protos::{JupiterSwapClient, JupiterSwapQuoteResponse, TokenInput};

use oracle_poc::{OracleData, OracleError, UpdateOracleParams, PROGRAM_SEED};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{
    get_ixn_discriminator,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::pubkey,
};
use tokio::try_join;

pub async fn fetch_jupiter_prices(
    runner: &FunctionRunner,
) -> Result<(JupiterSwapQuoteResponse, JupiterSwapQuoteResponse)> {
    // fetch Pyth price
    let x = JupiterSwapClient::new(None);

    let sol_token = TokenInput {
        address: "So11111111111111111111111111111111111111112".to_string(),
        decimals: 9,
    };

    let usdc_token = TokenInput {
        address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        decimals: 6,
    };

    let base_fut = x.get_quote(&sol_token, &usdc_token, "100", None);
    let quote_fut = x.get_quote(&usdc_token, &sol_token, "100", None);
    let quote_results = try_join!(base_fut, quote_fut);

    if quote_results.is_err() {
        runner.emit_error(33).await?;
    };

    let ur = quote_results.unwrap();
    println!("{:?}, {:?}", ur.0, ur.1);

    Ok(ur)
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
