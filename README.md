
# Solana NFT Marketplace

A decentralized NFT marketplace built on Solana. Users can mint, list, buy, and sell NFTs using their Solana wallet.

## 🚀 Features

- Connect with Solana wallet (e.g., Phantom)
- Mint new NFTs on Solana using Metaplex
- List NFTs for sale via Metaplex Auction House
- Buy NFTs from the marketplace
- View owned and listed NFTs

## 🛠️ Tech Stack

- **Frontend**: React, TypeScript, Solana Wallet Adapter
- **Blockchain**: Solana
- **NFT Interaction**: Metaplex
- **Wallet**: Phantom Wallet (or other Solana wallets)
- **Backend**: Rust (Solana backend)

## 📦 Installation

### Prerequisites

- Node.js and npm installed
- Solana wallet extension (e.g., Phantom)
- Docker and Docker Compose for the backend
- Rust and Cargo for backend compilation

### Getting Started

#### Frontend

1. **Clone the repository:**

   ```bash
   git clone https://github.com/mithesh06/nft-marketplace.git
   cd nft-marketplace
   ```

2. **Install dependencies:**

   ```bash
   npm install
   ```

3. **Run the development server:**

   ```bash
   npm run dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) in your browser.

#### Backend

1. **Navigate to the `nft_marketplace_backend` folder:**

   ```bash
   cd nft_marketplace_backend
   ```

2. **Run the backend with Docker:**

   From the `nft_marketplace_backend` directory, run:

   ```bash
   docker-compose up
   ```

   This will start the backend services defined in `docker-compose.yml`.

3. The backend API should now be available at [http://localhost:8080](http://localhost:8080).


## 📜 License

This project is licensed under the MIT License.
