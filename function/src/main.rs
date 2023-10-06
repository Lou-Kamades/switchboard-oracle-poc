pub use switchboard_solana::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use switchboard_solana::get_ixn_discriminator;

pub async fn perform(runner: &FunctionRunner) -> Result<()> {

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should be under 700 bytes after serialization
    let ix = create_pong_ix(runner);

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

pub fn create_ping_ix(runner: &FunctionRunner) -> Instruction {
    println!("ping 2: electric boogaloo");
      Instruction {
        program_id: oracle_poc::ID,
        accounts: vec![
                AccountMeta::new_readonly(runner.function, false),
                // our enclave generated signer must sign to update our program
                AccountMeta::new_readonly(runner.signer, true),
        ],
        data: get_ixn_discriminator("ping").to_vec()
        }
}

pub fn create_pong_ix(runner: &FunctionRunner) -> Instruction {
    println!("pong ix");
      Instruction {
        program_id: oracle_poc::ID,
        accounts: vec![
                AccountMeta::new_readonly(runner.signer, true),
        ],
        data: get_ixn_discriminator("pong").to_vec()
        }
}

pub fn create_update_ix(runner: &FunctionRunner) -> Instruction {
    let (oracle_key , _) = Pubkey::find_program_address(&[b"oracle"], &oracle_poc::ID);

        Instruction {
        program_id: oracle_poc::ID,
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
        data: get_ixn_discriminator("update_oracle").to_vec()
    }
}

#[cfg(test)]
mod tests {
    use switchboard_solana::FunctionRunner;

    use crate::{perform, create_update_ix, get_ixn_discriminator};

    // #[test]
    // fn mock() {

    //     std::env::set_var("CLUSTER", "devnet");
    //     std::env::set_var("function_key", "J4NqiGfeepFbbvNr6vxx4YMfTZ3DxtztF15UefaV4vkb");
    //     std::env::set_var("payer", "9UsqrFbrsKSo6CVApeXSr6BXHFQV9YJMLUyb7rR783gu");
    //     std::env::set_var("verifier", "A2FM9UNDG39deByDRgdLgmKxyTx1DCWUX1RfMCCN3gD3"); // ??
    //     std::env::set_var("reward_receiver", "DV9cTxjFYxCAKRhryFZEKhNJFFGCx7tzDqVF5cwanZCV"); // ??
    //     // println!("{}", &std::env::var("CLUSTER").unwrap());

    //     let runner = FunctionRunner::from_env(None).unwrap();
    //     println!("{}", runner);

    //     if runner.assert_mr_enclave().is_err() {
    //         panic!("199");
    //     }

    //     let ix = create_update_ix(&runner);

    //     println!("{:?}", ix);

    // }

    #[test]
    fn goo() {
        let x = get_ixn_discriminator("ping").to_vec();
        println!("{:?}", x);
    }
}