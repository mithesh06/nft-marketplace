use utoipa::OpenApi;
use crate::handlers::{
    register_wallet, create_listing, get_active_listings, create_transaction, get_transactions,
    health_check,
};
use crate::entities::{WalletRegisterRequest, ListingCreateRequest, TransactionCreateRequest};
use crate::models::{Listing, Transaction};

#[derive(OpenApi)]
#[openapi(
    paths(
        register_wallet,
        create_listing,
        get_active_listings,
        create_transaction,
        get_transactions,
        health_check,
    ),
    components(
        schemas(
            WalletRegisterRequest,
            ListingCreateRequest,
            TransactionCreateRequest,
            Listing,
            Transaction,
        )
    ),
    tags(
        (name = "NFT Marketplace", description = "API for managing wallet registration, listings, and transactions.")
    )
)]
pub struct ApiDoc;
