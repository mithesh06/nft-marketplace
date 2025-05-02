use actix_web::{web, HttpResponse, Responder};
use sqlx::{Acquire, PgPool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, Listing, Transaction};
use crate::entities::{ConfirmListingRequest, ConfirmTransactionRequest, ListingCreateRequest, RegisterNftRequest, TransactionCreateRequest, WalletRegisterRequest};
//use actix_web::Error;

// Health Check Endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json({
        serde_json::json!({ "status": "ok", "message": "NFT Marketplace Backend is running" })
    })
}

// Register Wallet Endpoint
pub async fn register_wallet(
    pool: web::Data<PgPool>,
    req: web::Json<WalletRegisterRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let result = sqlx::query(
        "INSERT INTO users (id, wallet_address, created_at) VALUES ($1, $2, $3) RETURNING id, wallet_address, created_at"
    )
    .bind(id)
    .bind(req.address.clone())
    .bind(now)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            let user = User {
                id: row.get("id"),
                wallet_address: row.get("wallet_address"),
                created_at: row.get("created_at"),
            };
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to register wallet")
        }
    }
}

// Create Listing Endpoint (Buy or Rent)
pub async fn create_listing(
    pool: web::Data<PgPool>,
    req: web::Json<ListingCreateRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let updated_at = now;

    let result = sqlx::query(
        "INSERT INTO listings (id, nft_id, seller_id, price_lamports, is_rentable, rent_price_lamports, rent_duration_days, status, created_at, updated_at) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
        RETURNING id, nft_id, seller_id, price_lamports, is_rentable, rent_price_lamports, rent_duration_days, status, created_at, updated_at"
    )
    .bind(id)
    .bind(req.nft_id)
    .bind(req.seller_id)
    .bind(req.price_lamports)
    .bind(req.is_rentable)
    .bind(req.rent_price_lamports)
    .bind(req.rent_duration_days)
    .bind("active") // Defaulting to active status
    .bind(now)
    .bind(updated_at)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            let listing = Listing {
                id: row.get("id"),
                nft_id: row.get("nft_id"),
                seller_id: row.get("seller_id"),
                price_lamports: row.get("price_lamports"),
                is_rentable: row.get("is_rentable"),
                rent_price_lamports: row.get("rent_price_lamports"),
                rent_duration_days: row.get("rent_duration_days"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            HttpResponse::Ok().json(listing)
        }
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create listing")
        }
    }
}

// Get Active Listings Endpoint
pub async fn get_active_listings(pool: web::Data<PgPool>) -> impl Responder {
    let listings = sqlx::query(
        "SELECT * FROM listings WHERE status = 'active'"
    )
    .fetch_all(pool.get_ref())
    .await;

    match listings {
        Ok(rows) => {
            let active_listings: Vec<Listing> = rows
                .into_iter()
                .map(|row| Listing {
                    id: row.get("id"),
                    nft_id: row.get("nft_id"),
                    seller_id: row.get("seller_id"),
                    price_lamports: row.get("price_lamports"),
                    is_rentable: row.get("is_rentable"),
                    rent_price_lamports: row.get("rent_price_lamports"),
                    rent_duration_days: row.get("rent_duration_days"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
                .collect();

            HttpResponse::Ok().json(active_listings)
        }
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch active listings")
        }
    }
}

// Create Transaction (Buy or Rent NFT)
pub async fn create_transaction(
    pool: web::Data<PgPool>,
    req: web::Json<TransactionCreateRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let timestamp = Utc::now().naive_utc();

    // Start a new transaction
    let mut transaction = pool.begin().await.unwrap();

    // Acquire a connection from the transaction to use as the executor
    let conn = transaction.acquire().await.unwrap();

    // Use the connection for the query execution
    let result = sqlx::query(
        "INSERT INTO transactions (id, nft_id, buyer_id, seller_id, type_, amount_lamports, timestamp, rent_start, rent_end) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        RETURNING id, nft_id, buyer_id, seller_id, type_, amount_lamports, timestamp, rent_start, rent_end"
    )
    .bind(id)
    .bind(req.nft_id)
    .bind(req.buyer_id)
    .bind(req.seller_id)
    .bind(req.type_.clone())
    .bind(req.amount_lamports)
    .bind(timestamp)
    .bind(req.rent_start)
    .bind(req.rent_end)
    .fetch_one(&mut *conn) // Use the connection for the query
    .await;

    match result {
        Ok(row) => {
            // Commit the transaction after the query completes successfully
            transaction.commit().await.unwrap();

            let transaction = Transaction {
                id: row.get("id"),
                nft_id: row.get("nft_id"),
                buyer_id: row.get("buyer_id"),
                seller_id: row.get("seller_id"),
                type_: row.get("type_"),
                amount_lamports: row.get("amount_lamports"),
                timestamp: row.get("timestamp"),
                rent_start: row.get("rent_start"),
                rent_end: row.get("rent_end"),
            };

            HttpResponse::Ok().json(transaction)
        }
        Err(e) => {
            eprintln!("DB Error: {}", e);
            // Rollback the transaction in case of an error
            transaction.rollback().await.unwrap();
            HttpResponse::InternalServerError().body("Failed to create transaction")
        }
    }
}



// Get Transactions Endpoint
pub async fn get_transactions(pool: web::Data<PgPool>) -> impl Responder {
    let transactions = sqlx::query(
        "SELECT * FROM transactions"
    )
    .fetch_all(pool.get_ref())
    .await;

    match transactions {
        Ok(rows) => {
            let all_transactions: Vec<Transaction> = rows
                .into_iter()
                .map(|row| Transaction {
                    id: row.get("id"),
                    nft_id: row.get("nft_id"),
                    buyer_id: row.get("buyer_id"),
                    seller_id: row.get("seller_id"),
                    type_: row.get("type_"),
                    amount_lamports: row.get("amount_lamports"),
                    timestamp: row.get("timestamp"),
                    rent_start: row.get("rent_start"),
                    rent_end: row.get("rent_end"),
                })
                .collect();

            HttpResponse::Ok().json(all_transactions)
        }
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch transactions")
        }
    }
}


// Register an NFT (owner must call this)
pub async fn register_nft(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterNftRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let result = sqlx::query(
        "INSERT INTO nfts (id, mint_address, metadata_uri, current_owner_id, is_rented, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(id)
    .bind(&req.mint_address)
    .bind(&req.metadata_uri)
    .bind(req.owner_id)
    .bind(false)
    .bind(now)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "NFT registered" })),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to register NFT")
        }
    }
}

// Record listing after Metaplex listing is completed
pub async fn list_for_sale(
    pool: web::Data<PgPool>,
    req: web::Json<ConfirmListingRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let result = sqlx::query(
        "INSERT INTO listings (id, nft_id, seller_id, price_lamports, is_rentable, rent_price_lamports, rent_duration_days, status, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'active', $8, $9)"
    )
    .bind(id)
    .bind(req.nft_id)
    .bind(req.seller_id)
    .bind(req.price_lamports)
    .bind(req.is_rentable)
    .bind(req.rent_price_lamports)
    .bind(req.rent_duration_days)
    .bind(now)
    .bind(now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "Listing saved" })),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to save listing")
        }
    }
}

// Confirm purchase/rent after Solana tx is signed
pub async fn confirm_transaction(
    pool: web::Data<PgPool>,
    req: web::Json<ConfirmTransactionRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let result = sqlx::query(
        "INSERT INTO transactions (id, nft_id, buyer_id, seller_id, type_, amount_lamports, timestamp, rent_start, rent_end)
         VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, NULL)"
    )
    .bind(id)
    .bind(req.nft_id)
    .bind(req.buyer_id)
    .bind(req.seller_id)
    .bind(&req.type_)
    .bind(req.amount_lamports)
    .bind(now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "Transaction recorded", "tx_signature": req.signature })),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Failed to record transaction")
        }
    }
}
