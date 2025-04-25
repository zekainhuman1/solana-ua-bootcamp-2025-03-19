use std::str::FromStr; // <--- ДОДАЙ ЦЕЙ ІМПОРТ

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::mint_to;

fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_file("/home/ubuntu/.config/solana/id.json").unwrap();

    let mint_pubkey = Pubkey::from_str("A8ALt1Rmk6xi8PFfUqXr7HamUP3BjuoLubUNLbK3WyWp").unwrap();
    let token_account_pubkey = Pubkey::from_str("4HEXakpP9wRoJofP7gnYH4brYAXP1kkn7FccCu3qU8Wx").unwrap();

    let mint_to_ix = mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &token_account_pubkey,
        &payer.pubkey(),
        &[],
        1_000_000_000, // 1 токен з 9 десятковими знаками
    )
    .unwrap();

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();

    println!("✅ Minted tokens! TX: {}", sig);
}
