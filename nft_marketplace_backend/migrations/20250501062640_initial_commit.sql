-- Add migration script here
-- migrations/20250501062640_initial_commit.sql

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE nfts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mint_address TEXT NOT NULL UNIQUE,
    metadata_uri TEXT,
    current_owner_id UUID NOT NULL REFERENCES users(id),
    is_rented BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE listings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nft_id UUID NOT NULL REFERENCES nfts(id),
    seller_id UUID NOT NULL REFERENCES users(id),
    price_lamports BIGINT NOT NULL,
    is_rentable BOOLEAN NOT NULL,
    rent_price_lamports BIGINT,
    rent_duration_days INTEGER,
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nft_id UUID NOT NULL REFERENCES nfts(id),
    buyer_id UUID NOT NULL REFERENCES users(id),
    seller_id UUID NOT NULL REFERENCES users(id),
    type TEXT NOT NULL,
    amount_lamports BIGINT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT now(),
    rent_start TIMESTAMP,
    rent_end TIMESTAMP
);
