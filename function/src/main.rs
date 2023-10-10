use pyth_sdk::Price;
use pyth_sdk_solana::load_price_feed_from_account;
pub use switchboard_solana::prelude::*;
use switchboard_utils::protos::{JupiterSwapClient, JupiterSwapQuoteResponse, TokenInput};

use oracle_poc::UpdateOracleParams;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{get_ixn_discriminator, solana_client::nonblocking::rpc_client::RpcClient, solana_sdk::pubkey};
use tokio::try_join;

// TODO: function to fetch all existing oracles
// TODO: some criteria for deciding which oracles get an update

pub async fn perform(runner: &FunctionRunner, rpc_client: RpcClient) -> Result<()> {
    let jupiter_prices = fetch_jupiter_prices(runner).await?;
    println!("jup prices: {:?}", jupiter_prices);
    
    // failover fetch Orca price?

    let pyth_usd_price = fetch_usd_price_from_pyth(rpc_client, runner).await?;

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should be under 700 bytes after serialization
    let ix = create_update_ix(runner, pyth_usd_price);

    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    runner.emit(vec![ix]).await?;
    Ok(())
}

#[tokio::main(worker_threads = 12)]
async fn main() -> Result<()> {
    // First, initialize the runner instance with a freshly generated Gramine keypair
    let runner = FunctionRunner::from_env(None)?;
    if runner.assert_mr_enclave().is_err() {
        runner.emit_error(199).await?;
    }

    let rpc_url = "http:/pythnet.rpcpool.com".to_string();
    let rpc_client = RpcClient::new(rpc_url);

    let res = perform(&runner, rpc_client).await;
    if let Some(e) = res.err() {
        runner.emit_error(1).await?;
    }
    Ok(())
}

pub fn create_update_ix(runner: &FunctionRunner, pyth_price: Price) -> Instruction {
    let (oracle_key, _) = Pubkey::find_program_address(&[b"oracle"], &oracle_poc::ID);
    let params = UpdateOracleParams { 
        price_raw: pyth_price.price,
        publish_time: pyth_price.publish_time, 
     };

    Instruction {
        program_id: oracle_poc::ID,
        accounts: vec![
            AccountMeta::new_readonly(runner.function, false),
            AccountMeta::new(oracle_key, false),
            // our enclave generated signer must sign to update our program
            AccountMeta::new_readonly(runner.signer, true),
        ],
        data: [
                get_ixn_discriminator("update_oracle").to_vec(),
                params.try_to_vec().unwrap(),
            ]
            .concat(),
    }
}

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

pub async fn fetch_orca_prices(
    _runner: &FunctionRunner,
) -> Result<()> {
    unimplemented!("LpExchangeRateTask");
}

pub async fn fetch_usd_price_from_pyth(
    rpc_client: RpcClient,
    runner: &FunctionRunner,
) -> Result<Price> {
    let usdc_price_key: Pubkey = pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");

    let account_result = rpc_client.get_account(&usdc_price_key).await;
    if account_result.is_err() {
        runner.emit_error(21).await?;
    };
    let mut usdc_price_account = account_result.unwrap();

    let feed_result =
        load_price_feed_from_account(&usdc_price_key, &mut usdc_price_account);
    if feed_result.is_err() {
        runner.emit_error(22).await?;
    };
    let usdc_price_feed = feed_result.unwrap();
    let price = usdc_price_feed.get_price_unchecked();
    Ok(price)
}

#[cfg(test)]
mod tests {
    use switchboard_solana::{FunctionRunner, solana_client::nonblocking::rpc_client::RpcClient};

    use crate::{create_update_ix, fetch_jupiter_prices, get_ixn_discriminator, perform, Result, fetch_usd_price_from_pyth};

    fn setup_runner() -> Result<FunctionRunner> {
        std::env::set_var("CLUSTER", "devnet");
        std::env::set_var(
            "function_key",
            "J4NqiGfeepFbbvNr6vxx4YMfTZ3DxtztF15UefaV4vkb",
        );
        std::env::set_var("payer", "9UsqrFbrsKSo6CVApeXSr6BXHFQV9YJMLUyb7rR783gu");
        std::env::set_var("verifier", "A2FM9UNDG39deByDRgdLgmKxyTx1DCWUX1RfMCCN3gD3"); // ??
        std::env::set_var(
            "reward_receiver",
            "DV9cTxjFYxCAKRhryFZEKhNJFFGCx7tzDqVF5cwanZCV",
        ); // ??
        let runner = FunctionRunner::from_env(None)?;
        Ok(runner)
    }

    // #[test]
    // fn mock() {

    //     // println!("{}", &std::env::var("CLUSTER").unwrap());
    //
    //     println!("{}", runner);

    //     if runner.assert_mr_enclave().is_err() {
    //         panic!("199");
    //     }

    //     let ix = create_update_ix(&runner);

    //     println!("{:?}", ix);

    // }

    #[tokio::test]
    async fn test_fetch_jupiter_price() {
        let runner = setup_runner().unwrap();
        let x = fetch_jupiter_prices(&runner).await;
        x.unwrap();
    }

    #[tokio::test]
    async fn test_fetch_pyth_price() {
        let rpc_url = "http:/pythnet.rpcpool.com".to_string();
        let rpc_client = RpcClient::new(rpc_url);
        let runner = setup_runner().unwrap();
        let price = fetch_usd_price_from_pyth(rpc_client, &runner).await.unwrap();
        println!("{:?}", price);
    }
}
