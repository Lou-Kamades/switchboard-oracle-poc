
use pyth_sdk::Price;


pub use switchboard_solana::prelude::*;


pub mod oracles;
pub mod prices;

use oracle_poc::{OracleData, UpdateOracleParams, PROGRAM_SEED};

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::{
    get_ixn_discriminator,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
    },
};


use crate::{
    oracles::fetch_oracle_by_name,
    prices::{fetch_jupiter_prices, fetch_usd_price_from_pyth},
};

// TODO: function to fetch all existing oracles
// TODO: some criteria for deciding which oracles get an update

pub async fn perform(runner: &FunctionRunner, rpc_client: RpcClient) -> Result<()> {
    let jupiter_prices = fetch_jupiter_prices(runner).await?;
    println!("jup prices: {:?}", jupiter_prices);

    // failover fetch Orca price?

    let pyth_usd_price = fetch_usd_price_from_pyth(&rpc_client, runner).await?;

    let oracle = fetch_oracle_by_name(&rpc_client, "New3".to_string()).await?; // TODO: env var?

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should be under 700 bytes after serialization
    let ix = create_update_ix(runner, pyth_usd_price, oracle);

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
    if let Some(_e) = res.err() {
        runner.emit_error(1).await?;
    }
    Ok(())
}

pub fn create_update_ix(
    runner: &FunctionRunner,
    pyth_price: Price,
    oracle: OracleData,
) -> Instruction {
    let (oracle_key, _) = Pubkey::find_program_address(&[&oracle.name[..]], &oracle_poc::ID);

    let (program_state, _) = Pubkey::find_program_address(&[PROGRAM_SEED], &oracle_poc::ID);
    let params = UpdateOracleParams {
        price_raw: pyth_price.price,
        publish_time: pyth_price.publish_time,
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
    use switchboard_solana::{solana_client::nonblocking::rpc_client::RpcClient, FunctionRunner};

    use crate::{
        create_update_ix, fetch_all_oracles, fetch_jupiter_prices, fetch_usd_price_from_pyth,
        get_ixn_discriminator, perform, Result,
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
    // async fn test_fetch_jupiter_price() {
    //     let runner = setup_runner().unwrap();
    //     let x = fetch_jupiter_prices(&runner).await;
    //     x.unwrap();
    // }

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
}
