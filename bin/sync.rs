use std::env::var;

use dotenvy::dotenv;
use sqlx::MySqlPool;

use ta::algolia::client::sync;
use ta::typecho::query::get_all_posts;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let pool: MySqlPool = MySqlPool::connect(&var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let posts = get_all_posts(&pool).await.expect("Failed to get all posts");

    sync("typecho_posts", &posts)
        .await
        .expect("Failed to sync posts to Algolia");
}
