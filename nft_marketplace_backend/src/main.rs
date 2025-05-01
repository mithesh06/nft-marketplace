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
            .route("/health", web::get().to(health_check))
            .route("/register_wallet", web::post().to(register_wallet))
            .route("/create_listing", web::post().to(create_listing))
            .route("/active_listings", web::get().to(get_active_listings))
            .route("/create_transaction", web::post().to(create_transaction))
            .route("/transactions", web::get().to(get_transactions))
    })
    .bind(addr)?
    .run()
    .await
}
