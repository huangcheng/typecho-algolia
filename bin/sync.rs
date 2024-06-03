use std::env::current_dir;
use std::env::var;
use std::fs::{read_to_string, write};

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

    let fingerprint_file = current_dir().unwrap().join("fingerprint");

    let hash = read_to_string(&fingerprint_file).unwrap_or("".to_string());

    let posts_json = serde_json::to_string(&posts).expect("Failed to serialize posts to JSON");

    let md5 = md5::compute(posts_json);

    if format!("{:x}", md5) == hash {
        println!("No need to sync posts to Algolia");

        return;
    }

    write(&fingerprint_file, format!("{:x}", md5)).expect("Failed to write finger");

    sync("typecho_posts", &posts)
        .await
        .expect("Failed to sync posts to Algolia");
}
