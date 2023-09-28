pub use switchboard_solana::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn perform(runner: &FunctionRunner) -> Result<()> {
    msg!("function runner loaded!");

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should  be under 700 bytes after serialization
    let ix = create_update_ix(runner);

    msg!("sending transaction");

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

    let res = perform(&runner).await;
    if let Some(e) = res.err() {
        runner.emit_error(1).await?;
    }
    Ok(())
}

pub fn create_update_ix(runner: &FunctionRunner) -> Instruction {
    let (oracle_key , _) = Pubkey::find_program_address(&[b"oracle"], &fat_oracle::ID);

        Instruction {
        program_id: fat_oracle::ID,
        accounts: vec![
            AccountMeta {
                pubkey: runner.function,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: oracle_key,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: runner.signer,
                is_signer: true,
                is_writable: false,
            },
        ],
        data: ix_discriminator("update_oracle").to_vec()
    }
}

pub fn ix_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
