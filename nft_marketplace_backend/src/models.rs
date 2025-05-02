use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub wallet_address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Nft {
    pub id: Uuid,
    pub mint_address: String,
    pub metadata_uri: Option<String>,
    pub current_owner_id: Uuid,
    pub is_rented: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Listing {
    pub id: Uuid,
    pub nft_id: Uuid,
    pub seller_id: Uuid,
    pub price_lamports: i64,
    pub is_rentable: bool,
    pub rent_price_lamports: Option<i64>,
    pub rent_duration_days: Option<i32>,
    pub status: String, // 'active', 'sold', 'rented', 'cancelled'
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub nft_id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub type_: String, // 'sale' or 'rent'
    pub amount_lamports: i64,
    pub timestamp: NaiveDateTime,
    pub rent_start: Option<NaiveDateTime>,
    pub rent_end: Option<NaiveDateTime>,
}
