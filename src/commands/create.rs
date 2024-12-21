// mod utils;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::pubkey::Pubkey;
// use solana_sdk::signature::{KeyPair, Signer};
// use solana_sdk::transaction::Transaction;
// use spl_token::{instruction::initialize_mint, state::Mint};

// pub async fn create_token(name: String, symbol: String, decimals: u64) {
//     let client = RpcClient::new("https://api.testnet-beta.solana.com");
//     let payer = utils::load_key_pair_from_the_config();
//     let mint = KeyPair::new();

//     let rent_excepmtion = client
//         .get_minimum_balance_for_rent_exemption(Mint::LEN)
//         .expect("Failed to reach rent excemption amount");

//     let create_account_ixn = solana_sdk::system_instruction::create_account(
//         &payer.pubkey(),
//         &mint.pubkey(),
//         rent_excepmtion,
//         Mint::LEN as u64,
//         &spl_token::id()
//     );  
// }
pub fn create(name: String, symbol: String, decimals: u64) {
    println!(
        "Name: {:?}, Symbol: {:?}, Decimals: {:?}",
        name, symbol, decimals
    );
}
