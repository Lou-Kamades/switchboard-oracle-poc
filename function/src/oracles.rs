use anyhow::anyhow;
use oracle_poc::state::{OracleContainer, OracleData};
use solana_account_decoder::UiAccountEncoding;
pub use switchboard_solana::prelude::*;
use switchboard_solana::solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
};

use oracle_poc::ORACLE_SEED;

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn fetch_all_oracles(rpc_client: &RpcClient) -> Result<Vec<OracleData>> {
    let account_config =  RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        commitment: Some(rpc_client.commitment()),
        ..RpcAccountInfoConfig::default()
    };

    let (oracle_key, _bump) = Pubkey::find_program_address(&[ORACLE_SEED], &oracle_poc::id());
    println!("{:?}, {:?}", oracle_key, oracle_poc::id());
    let account = rpc_client.get_account_with_config(&oracle_key, account_config)
        .await?;
    let container = OracleContainer::try_deserialize(&mut &account.value.unwrap().data[..])?;
    Ok(container.oracles.to_vec())
}

pub async fn fetch_oracle_by_name(rpc_client: &RpcClient, name: String) -> Result<OracleData> {
    let oracles = fetch_all_oracles(rpc_client).await?;

    let mut oracle_buffer = [0u8; 16];
    let name_bytes = name.as_bytes();
    let len = name_bytes.len();
    if len > 16 {
        panic!("Oracle name should be <= 16 bytes");
    }
    oracle_buffer[..len].copy_from_slice(&name_bytes[..len]);

    let maybe_oracle = oracles.into_iter().find(|x| x.name == oracle_buffer);
    maybe_oracle.ok_or(
        anyhow!(
            "Could not find Oracle with name: {} in {}",
            name,
            oracle_poc::id()
        )
        .into(),
    )
}

pub async fn fetch_oracles_by_name(rpc_client: &RpcClient, names: Vec<String>) -> Result<Vec<OracleData>> {
    let oracles = fetch_all_oracles(rpc_client).await?;

    let buffers: Vec<[u8; 16]> = names.into_iter().map(|n| {
        let mut oracle_buffer = [0u8; 16];
        let name_bytes = n.as_bytes();
        let len = name_bytes.len();
        if len > 16 {
            panic!("Oracle name should be <= 16 bytes");
        }
        oracle_buffer[..len].copy_from_slice(&name_bytes[..len]);
        oracle_buffer
    }).collect();

    let oracles: Vec<OracleData> = oracles.into_iter().filter(|x| buffers.contains(&x.name)).collect();
    Ok(oracles)
}
