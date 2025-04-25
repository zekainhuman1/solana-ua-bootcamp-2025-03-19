use mpl_token_metadata::instruction::{create_metadata_accounts_v3, CreateMetadataAccountsV3Args};
use mpl_token_metadata::state::{DataV2, Creator};
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;

use solana_program::pubkey::Pubkey;
use spl_token::state::{Mint, Account};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};


fn main() {
    // Примірники ключів (в реальному коді заміни на справжні)
    let metadata_pda = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    let mint_authority = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let update_authority = Pubkey::new_unique();
    let creator_pubkey = Pubkey::new_unique();

    let instruction = create_metadata_accounts_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_pda,
        mint_pubkey,
        mint_authority,
        payer,
        update_authority,
        CreateMetadataAccountsV3Args {
            data: DataV2 {
                name: "ExampleToken".to_string(),
                symbol: "EXT".to_string(),
                uri: "https://example.com/token.json".to_string(),
                seller_fee_basis_points: 500,
                creators: Some(vec![Creator {
                    address: creator_pubkey,
                    verified: true,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    );

    println!("Instruction created: {:?}", instruction);
}
