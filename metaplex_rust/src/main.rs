use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use mpl_auction_house::{
    instruction::buy,
    ID as AUCTION_HOUSE_PROGRAM_ID,
};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    // RPC client
    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Buyer keypair
    let buyer = Keypair::from_file("~/.config/solana/id.json")?;

    // Auction house + NFT details
    let auction_house_address = Pubkey::from_str("YourAuctionHouseAddress")?;
    let treasury_mint = Pubkey::from_str("So11111111111111111111111111111111111111112")?; // Native SOL
    let nft_mint = Pubkey::from_str("NFTMintAddress")?;
    let token_account = spl_associated_token_account::get_associated_token_address(&buyer.pubkey(), &nft_mint);

    let seller = Pubkey::from_str("SellerWalletAddress")?;
    let metadata = mpl_token_metadata::pda::find_metadata_account(&nft_mint).0;

    let price = 1_000_000_000; // 1 SOL
    let token_size = 1;

    // Derive trade state PDA
    let (buyer_trade_state, _) = Pubkey::find_program_address(
        &[
            b"auction_house",
            auction_house_address.as_ref(),
            buyer.pubkey().as_ref(),
            treasury_mint.as_ref(),
            token_account.as_ref(),
            nft_mint.as_ref(),
            &price.to_le_bytes(),
            &token_size.to_le_bytes(),
        ],
        &AUCTION_HOUSE_PROGRAM_ID,
    );

    // Instruction
    let buy_ix = buy(
        buyer.pubkey(),
        auction_house_address,
        token_account,
        treasury_mint,
        nft_mint,
        metadata,
        buyer_trade_state,
        seller,
        price,
        token_size,
    );

    // Transaction
    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[buy_ix],
        Some(&buyer.pubkey()),
        &[&buyer],
        blockhash,
    );

    // Send
    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("Buy tx submitted: {}", sig);

    Ok(())
}
