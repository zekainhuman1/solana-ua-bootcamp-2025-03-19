use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{read_keypair_file, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};
use std::str::FromStr;

fn main() {
    // 1. RPC клієнт (можна замінити на devnet/mainnet)
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // 2. Зчитування keypair
    let payer = read_keypair_file("/home/ubuntu/.config/solana/id.json")
        .expect("Failed to read keypair");

    let recipient = Pubkey::from_str("6xEa5tzFTjkaVji7r6t6mpMxoVf7W8Sooi8dbcj3wirv")
        .expect("Invalid recipient pubkey");

    // 4. Скільки SOL надсилаємо (0.01 SOL = 10_000_000 лампортів)
    let lamports = 10_000_000;

    // 5. Створення інструкції на переказ
    let instruction = system_instruction::transfer(&payer.pubkey(), &recipient, lamports);

    // 6. Отримання останнього hash блоку
    let recent_blockhash = client.get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // 7. Транзакція
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // 8. Відправка
    let signature = client.send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("✅ Transaction successful!");
    println!("🔗 https://explorer.solana.com/tx/{}?cluster=devnet", signature);
}
