use crate::utils::read_keys_from_file;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::Keypair, signer::Signer, system_instruction, transaction::Transaction,
};
use spl_token::{instruction::initialize_mint, ID as SPL_TOKEN_PROGRAM_ID};

pub async fn create() {
    let payer_in_bytes =
        read_keys_from_file().expect("Failed to read the Key Pairs from the config file");
    let client = RpcClient::new("https://api.devnet.solana.com");
    let payer = match Keypair::from_bytes(&payer_in_bytes) {
        Ok(key_pair) => key_pair,
        Err(err) => {
            println!("Failed to read key pair from the bytes: {}", err);
            return;
        }
    };
    let mint_key_pair = Keypair::new();

    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => {
            println!("Failed to get the latest blockhash: {}", err);
            return;
        }
    };
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_key_pair.pubkey(),
        1_000_000_000,
        82,
        &SPL_TOKEN_PROGRAM_ID,
    );
    let initialize_mint_ix = match initialize_mint(
        &SPL_TOKEN_PROGRAM_ID,
        &mint_key_pair.pubkey(),
        &payer.pubkey(),
        None,
        9,
    ) {
        Ok(ixn) => ixn,
        Err(err) => {
            println!("Failed to create initialize mint instruction : {}", err);
            return;
        }
    };
    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
    );
    transaction
        .try_sign(&[&payer, &mint_key_pair], recent_blockhash)
        .expect("Failed to sign the transaction");
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Signature: {}", signature);
            println!("New SPL-Token Address: {}", mint_key_pair.pubkey());
        }
        Err(err) => {
            println!("Failed to execute the transaction: {}", err);
        }
    }
}
