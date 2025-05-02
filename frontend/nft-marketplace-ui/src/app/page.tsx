'use client';

import { FC, useState } from 'react';
import Head from 'next/head';
import Header from './components/Header';
import SellNFT from './components/SellNFT';

const Home: FC = () => {
  const [isSellSelected, setIsSellSelected] = useState(false);

  return (
    <>
      <Head>
        <title>Solana NFT Marketplace</title>
        <meta name="description" content="A Solana-based NFT marketplace" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <Header setIsSellSelected={setIsSellSelected} />
        {isSellSelected && <SellNFT />}
        {/* Add additional sections here if needed */}
      </main>
    </>
  );
};

export default Home;
