use crate::utils::read_keys_from_file;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::{instruction::transfer, ID as SPL_TOKEN_PROGRAM_ID};
use std::str::FromStr;

pub async fn transfer_tokens(token: String, amount: u64, recipient: String) {
    let payer_in_bytes =
        read_keys_from_file().expect("Failed to read Key Pairs from the config file");
    let client = RpcClient::new("https://api.devnet.solana.com");
    let payer = Keypair::from_bytes(&payer_in_bytes).expect("Invalid keypair bytes");

    let mint_pub_key = Pubkey::from_str(&token).expect("Invalid token address");
    let recipient_pub_key = Pubkey::from_str(&recipient).expect("Invalid recipient address");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to fetch blockhash");

    let source_ata = get_associated_token_address(&payer.pubkey(), &mint_pub_key);
    let recipient_ata = get_associated_token_address(&recipient_pub_key, &mint_pub_key);

    let mut instructions = vec![];

    if client.get_account(&source_ata).is_err() {
        instructions.push(create_associated_token_account(
            &payer.pubkey(),
            &payer.pubkey(),
            &mint_pub_key,
            &SPL_TOKEN_PROGRAM_ID,
        ));
    }

    if client.get_account(&recipient_ata).is_err() {
        instructions.push(create_associated_token_account(
            &payer.pubkey(),
            &recipient_pub_key,
            &mint_pub_key,
            &SPL_TOKEN_PROGRAM_ID,
        ));
    }

    let transfer_ix = transfer(
        &SPL_TOKEN_PROGRAM_ID,
        &source_ata,
        &recipient_ata,
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
    )
    .expect("Failed to create transfer instruction");
    instructions.push(transfer_ix);
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    transaction
        .try_sign(&[&payer], recent_blockhash)
        .expect("Failed to sign transaction");
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction succeeded with signature: {}", signature),
        Err(err) => println!("Transaction failed: {:?}", err),
    }
}
