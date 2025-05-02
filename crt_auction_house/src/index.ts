import { Connection, clusterApiUrl, Keypair } from '@solana/web3.js';
import { Metaplex, keypairIdentity } from '@metaplex-foundation/js';
import fs from 'fs';
import path from 'path';

async function main() {
  const connection = new Connection(clusterApiUrl('devnet'));

  // Dynamically read and parse the secret key JSON
  const secretKeyPath = path.resolve(__dirname, '../../mint_nfts/nft-keypair.json');
  const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(secretKeyPath, 'utf-8')));
  const keypair = Keypair.fromSecretKey(secretKey);

  const metaplex = Metaplex.make(connection).use(keypairIdentity(keypair));

  const { auctionHouse } = await metaplex
    .auctionHouse()
    .create({
      sellerFeeBasisPoints: 200, // 2% fee
      requiresSignOff: false,
      canChangeSalePrice: true,
    });

  console.log('Auction House Created:', auctionHouse.address.toBase58());
}

main();
