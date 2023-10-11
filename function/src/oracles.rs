use anyhow::anyhow;
use oracle_poc::OracleData;
use solana_account_decoder::UiAccountEncoding;
pub use switchboard_solana::prelude::*;
use switchboard_solana::solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
};

#[allow(hidden_glob_reexports)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn fetch_all_oracles(rpc_client: &RpcClient) -> Result<Vec<OracleData>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            0,
            OracleData::discriminator().to_vec(),
        ))]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(rpc_client.commitment()),
            ..RpcAccountInfoConfig::default()
        },
        with_context: Some(true),
    };

    let program_accounts = rpc_client
        .get_program_accounts_with_config(&oracle_poc::id(), config)
        .await?;
    let oracles: Vec<OracleData> = program_accounts
        .into_iter()
        .map(|r| OracleData::try_deserialize(&mut &r.1.data[..]).unwrap())
        .collect();

    Ok(oracles)
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
