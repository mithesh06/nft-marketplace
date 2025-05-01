use serde::Deserialize;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct WalletRegisterRequest {
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct ListingCreateRequest {
    pub nft_id: Uuid,
    pub seller_id: Uuid,
    pub price_lamports: i64,
    pub is_rentable: bool,
    pub rent_price_lamports: Option<i64>,
    pub rent_duration_days: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionCreateRequest {
    pub nft_id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub type_: String, // 'sale' or 'rent'
    pub amount_lamports: i64,
    pub rent_start: Option<NaiveDateTime>,
    pub rent_end: Option<NaiveDateTime>,
}
