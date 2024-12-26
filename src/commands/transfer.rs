// use std::str::FromStr;

// use crate::utils::read_keys_from_file;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
// use spl_token::{instruction::transfer, ID as SPL_TOKEN_PROGRAM_ID, state::Mint};
// pub async fn transfer_tokens(token: String, amount: u64, recipient: String) {
//     let payer_in_bytes =
//         read_keys_from_file().expect("Failed to read the Key Pairs from the config file");
//     let client = RpcClient::new("https://api.devnet.solana.com");
//     let payer = match Keypair::from_bytes(&payer_in_bytes) {
//         Ok(key_pair) => key_pair,
//         Err(err) => {
//             println!("Failed to read key pair from the bytes: {}", err);
//             return;
//         }
//     };
//     let mint_pub_key = match Pubkey::from_str(&token) {
//         Ok(mint_pubkey) => mint_pubkey,
//         Err(err) => {
//             println!("Failed to read token address: {:?}", err);
//             return;
//         }
//     };

//     let recipient_pub_key = match Pubkey::from_str(&recipient) {
//         Ok(recipient_pubkey) => recipient_pubkey,
//         Err(err) => {
//             println!("Failed to read token address: {:?}", err);
//             return;
//         }
//     };
//     let recent_blockhash = match client.get_latest_blockhash() {
//         Ok(blockhash) => blockhash,
//         Err(err) => {
//             println!("Failed to get the latest blockhash: {}", err);
//             return;
//         }
//     };

//     let mint_account = match client.get_account(&mint_pub_key) {
//         Ok(account) => account,
//         Err(err) => {
//             println!("Failed to fetch mint account : {:?}", err);
//             return;
//         }
//     };

//     let mint_account_data = Mint::try_from_slice(&mint_account.data).expect("Failed to unpack mint account data");

//     let mint_ixn = match transfer(
//         &SPL_TOKEN_PROGRAM_ID,
//         &payer.pubkey(),
//         &recipient_pub_key,
//         mint_account.,
//         &[&payer.pubkey()],
//         amount,
//     ) {
//         Ok(ixn) => ixn,
//         Err(err) => {
//             println!("Failed to create mint instruction: {:?}", err);
//             return;
//         }
//     };
//     let mut transaction = Transaction::new_with_payer(&[mint_ixn], Some(&payer.pubkey()));

//     transaction
//         .try_sign(&[&payer], recent_blockhash)
//         .expect("Failed to sign the transaction");
//     match client.send_and_confirm_transaction(&transaction) {
//         Ok(signature) => {
//             println!("Signature: {}", signature);
//         }
//         Err(err) => {
//             println!("Failed to execute the transaction: {}", err);
//         }
//     }
// }

pub async fn transfer_tokens(token: String, amount: u64, recipient: String) {
    println!("token: {}, amount: {}, recepient: {}", token, amount, recipient);
}