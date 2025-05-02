use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use nft_marketplace_backend::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    // Run migrations
    sqlx::migrate!()
    .run(&pool)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    println!("Database migrations applied successfully.");

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);
    println!("Starting server at http://{}", addr);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // Pass pool to each handler
            .route("/health", web::get().to(handlers::health_check))
            .route("/wallet/register", web::post().to(handlers::register_wallet))
            .route("/nft/register", web::post().to(handlers::register_nft))
            .route("/listings", web::post().to(handlers::create_listing))
            .route("/listings/active", web::get().to(handlers::get_active_listings))
            .route("/transactions", web::get().to(handlers::get_transactions))
            .route("/transaction/create", web::post().to(handlers::create_transaction))
            .route("/transaction/confirm", web::post().to(handlers::confirm_transaction))
            .route("/listing/confirm", web::post().to(handlers::list_for_sale))
    })
    .bind(addr)?
    .run()
    .await
}
