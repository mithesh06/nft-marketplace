'use client';

import { FC, useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import {
  Metaplex,
  walletAdapterIdentity,
  irysStorage,
  sol,
  Metadata,
  Nft,
  Sft,
} from '@metaplex-foundation/js';

type DisplayNFT = Metadata | Nft | Sft;

const SellNFT: FC = () => {
  const { publicKey, connected, wallet } = useWallet();
  const [metaplex, setMetaplex] = useState<Metaplex | null>(null);
  const [ownedNFTs, setOwnedNFTs] = useState<DisplayNFT[]>([]);
  const [isListing, setIsListing] = useState<string | null>(null);
  const [price, setPrice] = useState<string>(''); // State for price input

  useEffect(() => {
    if (connected && wallet && publicKey) {
      const connection = new Connection('https://api.devnet.solana.com');
      const mx = Metaplex.make(connection)
        .use(walletAdapterIdentity(wallet.adapter))
        .use(
          irysStorage({
            address: 'https://devnet.irys.xyz',
            providerUrl: 'https://api.devnet.solana.com',
            timeout: 60000,
          })
        );
      setMetaplex(mx);
    }
  }, [connected, wallet, publicKey]);

  const fetchOwnedNFTs = async () => {
    if (!metaplex || !publicKey) return;
    try {
      const nfts = await metaplex.nfts().findAllByOwner({ owner: publicKey });
      setOwnedNFTs(nfts);
    } catch (error) {
      console.error('Failed to fetch NFTs:', error);
    }
  };

  useEffect(() => {
    fetchOwnedNFTs();
  }, [metaplex, publicKey]);

  const handleEnableListing = async (mintAddress: PublicKey) => {
    if (!metaplex || !publicKey || !price) return;

    const parsedPrice = parseFloat(price);
    if (isNaN(parsedPrice) || parsedPrice <= 0) {
      alert('Please enter a valid price.');
      return;
    }

    setIsListing(mintAddress.toBase58());

    try {
      const auctionHouseOutput = await metaplex
        .auctionHouse()
        .create({
          sellerFeeBasisPoints: 500,
          requiresSignOff: false,
          canChangeSalePrice: true,
        });

      const auctionHouse = auctionHouseOutput.auctionHouse;

      await metaplex.auctionHouse().list({
        auctionHouse,
        seller: publicKey,
        mintAccount: mintAddress,
        price: sol(parsedPrice),
      });

      alert('NFT enabled for listing!');
      fetchOwnedNFTs();
    } catch (error) {
      console.error('Enable listing failed:', error);
      alert('Failed to enable NFT listing.');
    } finally {
      setIsListing(null);
    }
  };

  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold text-white mb-6">Enable NFTs for Sale</h2>

      <div className="mb-4">
        <label htmlFor="price" className="block text-white mb-2">Set Price (in SOL)</label>
        <input
          id="price"
          type="number"
          value={price}
          onChange={(e) => setPrice(e.target.value)}
          placeholder="Enter price in SOL"
          className="w-full p-3 border border-gray-300 rounded-xl"
        />
      </div>

      <div className="flex gap-x-12 overflow-x-auto pb-6">
        {ownedNFTs.map((nft, index) => {
          const mintAddress =
            'mintAddress' in nft ? nft.mintAddress : new PublicKey(nft.mint);
          const imageUrl =
            'json' in nft && nft.json?.image
              ? nft.json.image
              : (nft as any).uri;

          return (
            <div
              key={index}
              className="bg-white text-black rounded-2xl shadow-lg p-6 min-w-[340px] max-w-[400px] flex-shrink-0"
            >
              <img
                src={imageUrl}
                alt={`NFT ${nft.name}`}
                className="mb-4 w-full h-64 object-cover rounded-xl"
              />
              <h3 className="text-xl font-semibold">{nft.name}</h3>
              <p className="text-gray-600 text-sm mb-4">{nft.symbol}</p>
              <button
                className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-4 rounded-xl"
                onClick={() => handleEnableListing(mintAddress)}
                disabled={isListing === mintAddress.toBase58()}
              >
                {isListing === mintAddress.toBase58()
                  ? 'Enabling...'
                  : 'Enable Listing'}
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default SellNFT;
