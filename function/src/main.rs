#![feature(async_fn_in_trait)]
use pyth_sdk::Price;

pub use switchboard_solana::prelude::*;

pub mod oracles;
pub mod prices;

use oracle_poc::{state::OracleData, UpdateOracleParams, ORACLE_SEED, PROGRAM_SEED};

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{
    get_ixn_discriminator, solana_client::nonblocking::rpc_client::RpcClient,
};
use switchboard_utils::protos::TokenInput;

use crate::{
    oracles::{fetch_oracle_by_name, fetch_oracles_by_name},
    prices::{fetch_jupiter_quotes, fetch_usd_price_from_pyth},
};

// TODO: function to fetch all existing oracles
// TODO: some criteria for deciding which oracles get an update

pub async fn perform(runner: &FunctionRunner, rpc_client: RpcClient) -> Result<()> {
    let sol_token = TokenInput {
        address: "So11111111111111111111111111111111111111112".to_string(),
        decimals: 9,
    };

    let jupiter_quotes = fetch_jupiter_quotes(runner, &sol_token).await?;

    // println!("jup prices: {:?}", jupiter_prices);
    println!("got jupiter price");

    // failover fetch Orca price?

    let pyth_usd_price = fetch_usd_price_from_pyth(&rpc_client, runner).await?;
    let oracle_names = vec!["1".to_string()];

    let devnet_url = "devnet-url".to_string();
    let devnet_client = RpcClient::new(devnet_url);

    println!("fetching oracles");
    let oracles = fetch_oracles_by_name(&devnet_client, oracle_names).await?; // TODO: env var?
    println!("oracles: {:?}", oracles);

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should be under 700 bytes after serialization
    let mut ixs = vec![];
    // for oracle in oracles { // todo : fix
    let ix = create_update_ix(runner, &pyth_usd_price, "1".to_string());
    println!("ix len: {:?}", ix.data.len());
    ixs.push(ix);
    // }

    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    runner.emit(ixs).await?;
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
    if let Some(_e) = res.err() {
        runner.emit_error(1).await?;
    }
    Ok(())
}

pub fn create_update_ix(
    runner: &FunctionRunner,
    pyth_price: &Price,
    oracle_name: String,
) -> Instruction {
    let (oracle_key, _) = Pubkey::find_program_address(&[ORACLE_SEED], &oracle_poc::ID);
    let (program_state, _) = Pubkey::find_program_address(&[PROGRAM_SEED], &oracle_poc::ID);
    let params = UpdateOracleParams {
        price_raw: pyth_price.price,
        oracle_name,
    };

    Instruction {
        program_id: oracle_poc::ID,
        accounts: vec![
            AccountMeta::new_readonly(program_state, false),
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

#[cfg(test)]
mod tests {
    use oracle_poc::ORACLE_SEED;
    use switchboard_solana::{
        solana_client::nonblocking::rpc_client::RpcClient, FunctionRunner, Pubkey,
    };
    use switchboard_utils::protos::TokenInput;

    use crate::{
        create_update_ix, fetch_usd_price_from_pyth, get_ixn_discriminator,
        oracles::{fetch_all_oracles, fetch_oracles_by_name},
        perform,
        prices::{
            calculate_avg_price_and_std_dev, estimate_price_from_quote, fetch_jupiter_prices,
            fetch_jupiter_quotes,
        },
        Result,
    };

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

    // #[tokio::test]
    // async fn mock() {
    //     let rpc_url = "http:/pythnet.rpcpool.com".to_string();
    //     let rpc_client = RpcClient::new(rpc_url);
    //     let runner = setup_runner().unwrap();
    //     if runner.assert_mr_enclave().is_err() {
    //         panic!("199");
    //     }

    //     let price = fetch_usd_price_from_pyth(rpc_client, &runner)
    //         .await
    //         .unwrap();

    //     let ix = create_update_ix(&runner, price);
    //     // println!("{:?}", ix);
    // }

    // #[tokio::test]
    // async fn test_fetch_jupiter_prices() {
    //     let x = fetch_jupiter_prices(vec![
    //         "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    //         "SOL".to_string(),
    //     ])
    //     .await;
    //     x.unwrap();
    // }

    #[tokio::test]
    async fn test_fetch_jupiter_quotes() {
        let runner = setup_runner().unwrap();
        let sol_token = TokenInput {
            address: "MangoCzJ36AjZyKwVj3VnYU4GTonjfVEnJmvvWaxLac".to_string(),
            decimals: 6,
        };

        // let sol_token = TokenInput {
        //     address: "So11111111111111111111111111111111111111112".to_string(),
        //     decimals: 9,
        // };

        let jupiter_quotes = fetch_jupiter_quotes(&runner, &sol_token).await.unwrap();

        let (mean, std_dev) = calculate_avg_price_and_std_dev(&jupiter_quotes, &sol_token);

        println!("{:?} {:?}", mean, std_dev);
        // for q in jupiter_quotes {
        //     println!("{:?}", estimate_price_from_quote(&q, &sol_token));
        //     println!("{:?}", q);
        // }
    }

    // #[tokio::test]
    // async fn test_fetch_pyth_price() {
    //     let rpc_url = "http:/pythnet.rpcpool.com".to_string();
    //     let rpc_client = RpcClient::new(rpc_url);
    //     let runner = setup_runner().unwrap();
    //     let price = fetch_usd_price_from_pyth(rpc_client, &runner)
    //         .await
    //         .unwrap();
    //     println!("{:?}", price);
    // }

    // #[tokio::test]
    // async fn test_fetch_oracles() {
    //     let rpc_url = "devnet-url".to_string();
    //     let rpc_client = RpcClient::new(rpc_url);
    //     let oracle_names = vec![
    //         "New3".to_string(),
    //         "New4".to_string(),
    //         "New5".to_string(),
    //         "New6".to_string(),
    //         "New7".to_string(),
    //     ];
    //     let goo = fetch_oracles_by_name(&rpc_client, oracle_names)
    //         .await
    //         .unwrap();
    //     println!("{:?}", goo);
    // }
}
