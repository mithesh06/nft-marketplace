// src/app/components/Header.tsx
'use client';

import { FC, useState } from 'react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useWallet } from '@solana/wallet-adapter-react';

interface HeaderProps {
  setIsSellSelected: (value: boolean) => void;
}

const Header: FC<HeaderProps> = ({ setIsSellSelected }) => {
  const [isSellSelected, setIsSellSelectedInternal] = useState(false);
  const { connected, publicKey } = useWallet();

  const handleSellClick = () => {
    if (!connected || !publicKey) {
      alert('Please connect your wallet to list NFTs for sale.');
      return;
    }
    setIsSellSelected(true);
    setIsSellSelectedInternal(true);
  };

  return (
    <header style={{ backgroundColor: '#111', padding: '1.5rem', color: '#fff', fontFamily: 'sans-serif' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        {/* Branding */}
        <h1 style={{ fontSize: '1.5rem', fontWeight: 'bold' }}>Solana NFT Marketplace</h1>

        {/* Buy/Sell Toggle */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <label htmlFor="buy-sell-toggle">Buy / Sell</label>
          <input
            type="radio"
            id="buy"
            name="buy-sell-toggle"
            value="buy"
            defaultChecked={!isSellSelected}
            onClick={() => {
              setIsSellSelected(false);
              setIsSellSelectedInternal(false);
            }}
          />
          <input
            type="radio"
            id="sell"
            name="buy-sell-toggle"
            value="sell"
            checked={isSellSelected}
            onClick={handleSellClick}
          />
        </div>

        {/* Connect Wallet Button */}
        <WalletMultiButton />
      </div>

      {/* Subtext */}
      <div style={{ marginTop: '2rem' }}>
        <h2 style={{ fontSize: '1.25rem', color: '#c084fc' }}>Explore NFTs</h2>
        <p style={{ color: '#ccc' }}>Discover and collect unique digital artwork.</p>
        <p
          style={{
            marginTop: '1rem',
            backgroundColor: '#222',
            padding: '0.75rem 1rem',
            borderRadius: '8px',
            color: '#aaa',
          }}
        >
          <strong>Tip:</strong> Connect your Phantom wallet to buy NFTs.
        </p>
      </div>
    </header>
  );
};

export default Header;
