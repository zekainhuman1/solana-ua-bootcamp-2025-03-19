// імпорти
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::{read_keypair_file, Signer}, transaction::Transaction};
use spl_associated_token_account::{get_associated_token_address};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::id as token_program_id;

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let payer = read_keypair_file("/home/ubuntu/.config/solana/id.json").expect("Missing keypair");

    let mint_pubkey = Pubkey::from_str("A8ALt1Rmk6xi8PFfUqXr7HamUP3BjuoLubUNLbK3WyWp").unwrap();
    let associated_account = get_associated_token_address(&payer.pubkey(), &mint_pubkey);

    let create_ata_ix = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_pubkey,
        &token_program_id(),
    );

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("✅ ATA created: {}", associated_account);
    println!("🔗 https://explorer.solana.com/tx/{}?cluster=devnet", sig);
}
