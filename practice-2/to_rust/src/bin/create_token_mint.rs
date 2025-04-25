use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction,
    system_program,
    program_pack::Pack, // ✅ Важливо для доступу до Mint::LEN
};
use spl_token::{instruction as token_instruction, state::Mint};

fn main() {
    // 1. RPC клієнт
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // 2. Ключ платника
    let payer = read_keypair_file("/home/ubuntu/.config/solana/id.json").expect("Failed to read keypair");

    // 3. Генерація нового mint акаунта
    let mint = Keypair::new();

    // 4. Обчислення вартості оренди
    let rent = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("Failed to get rent exemption");

    // 5. Створення акаунта mint
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    // 6. Ініціалізація токена
    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(), // Може бути owner або mint_authority
        None,            // freeze authority
        0,               // decimals
    )
    .expect("Failed to create initialize_mint instruction");

    // 7. Отримання хешу останнього блоку
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

    // 8. Транзакція
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    // 9. Відправка транзакції
    let signature = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    println!("✅ Token mint created successfully!");
    println!("🪙 Mint Address: {}", mint.pubkey());
    println!("🔗 https://explorer.solana.com/tx/{}?cluster=devnet", signature);
}
