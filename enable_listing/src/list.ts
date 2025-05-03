import {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  import {
    actions,
    programs,
  } from '@metaplex/js';
  import secretKey from './wallet.json'; // Make sure this file contains a 64-byte secret key array
  
  // 🔐 Load Keypair
  const keypair = Keypair.fromSecretKey(Uint8Array.from(secretKey));
  
  // ✅ Manual Metaplex wallet wrapper
  const wallet = {
    publicKey: keypair.publicKey,
    signTransaction: async (tx: Transaction) => {
      tx.partialSign(keypair);
      return tx;
    },
    signAllTransactions: async (txs: Transaction[]) => {
      return txs.map((tx) => {
        tx.partialSign(keypair);
        return tx;
      });
    },
  };
  
  // 🔗 Solana Devnet connection
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  
  // 📌 Replace these placeholders
  const auctionHouseAddress = new PublicKey('<AUCTION_HOUSE_ADDRESS>');
  const nftMintAddress = new PublicKey('<NFT_MINT_ADDRESS>');
  const userTokenAccount = new PublicKey('<USER_TOKEN_ACCOUNT>'); // Associated token account holding NFT
  const listingPrice = 1 * 1e9; // 1 SOL in lamports
  
  async function listNFT() {
    try {
      // 🏛 Load Auction House object
      const auctionHouse = await programs.auctionHouse.AuctionHouse.load(
        connection,
        auctionHouseAddress,
        wallet
      );
  
      // 📝 Create Listing Transaction
      const listingTx = await actions.createListing(
        connection,
        keypair,               // Payer
        auctionHouse,
        userTokenAccount,      // ATA where NFT is stored
        nftMintAddress,        // NFT mint
        listingPrice           // Listing price in lamports
      );
  
      // 🚀 Send Transaction
      const txId = await connection.sendTransaction(listingTx, [keypair], {
        skipPreflight: false,
        preflightCommitment: 'confirmed',
      });
  
      console.log(`✅ NFT listed successfully! Tx ID: ${txId}`);
    } catch (err) {
      console.error('❌ Failed to list NFT:', err);
    }
  }
  
  listNFT();
  