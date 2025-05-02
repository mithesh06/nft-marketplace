use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use solana_program::{program_pack::Pack, system_program, sysvar};
use spl_token::{
    instruction as token_instruction,
    state::Mint,
};
use spl_associated_token_account::get_associated_token_address;

use mpl_token_metadata::{
    ID as TOKEN_METADATA_PROGRAM_ID,
    types::{DataV2, Creator},
    instructions::CreateMetadataAccountV3Builder,
};

fn main() -> Result<()> {
    // 1) RPC & payer setup
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let payer = read_keypair_file("./nft-keypair.json").expect("Failed to read keypair file");
    let payer_pubkey = payer.pubkey();

    for i in 0..10 {
        println!("Minting NFT #{}", i + 1);

        // 2) Create new mint
        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();
        let token_account = get_associated_token_address(&payer_pubkey, &mint_pubkey);

        let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

        let mut instructions = vec![
            // a) allocate & init mint
            system_instruction::create_account(
                &payer_pubkey,
                &mint_pubkey,
                mint_rent,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            token_instruction::initialize_mint(
                &spl_token::id(),
                &mint_pubkey,
                &payer_pubkey,
                Some(&payer_pubkey),
                0,
            )?,
            // b) create ATA & mint 1 token
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer_pubkey,
                &payer_pubkey,
                &mint_pubkey,
                &spl_token::id(),
            ),
            token_instruction::mint_to(
                &spl_token::id(),
                &mint_pubkey,
                &token_account,
                &payer_pubkey,
                &[],
                1,
            )?,
        ];

        // 3) Derive the metadata PDA
        let (metadata_pda, _) = Pubkey::find_program_address(
            &[
                b"metadata".as_ref(),
                TOKEN_METADATA_PROGRAM_ID.as_ref(),
                mint_pubkey.as_ref(),
            ],
            &TOKEN_METADATA_PROGRAM_ID,
        );

        // 4) Build the `CreateMetadataAccountV3` instruction
        let metadata_ix = CreateMetadataAccountV3Builder::new()
            .metadata(metadata_pda)
            .mint(mint_pubkey)
            .mint_authority(payer_pubkey)
            .payer(payer_pubkey)
            .update_authority(payer_pubkey, true)
            .system_program(system_program::ID)
            .rent(Some(sysvar::rent::ID))
            .data(DataV2 {
                name: format!("MyNFT #{}", i + 1),
                symbol: "MYNFT".to_string(),
                uri: "https://example.com/nft.json".to_string(), // your metadata URI
                seller_fee_basis_points: 500,
                creators: Some(vec![Creator {
                    address: payer_pubkey,
                    verified: true,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            })
            .is_mutable(true)
            .instruction();

        instructions.push(metadata_ix);

        // 5) Send transaction
        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer_pubkey),
            &[&payer, &mint],
            recent_blockhash,
        );
        client.send_and_confirm_transaction(&tx)?;
        println!("NFT #{} minted at {}", i + 1, mint_pubkey);
    }

    Ok(())
}
