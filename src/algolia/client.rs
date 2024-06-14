use std::env::var;

use reqwest::{Client, StatusCode};

use crate::algolia::schema::{BatchAdd, IndexSetting, ListIndexes, UnsuccessfulResponse};
use crate::ClientError;
use crate::typecho::model::Post;

async fn add_abjects(index: &str, post: &Vec<Post>) -> Result<(), ClientError> {
    let api_key = var("ALGOLIA_API_KEY").unwrap();
    let app_id = var("ALGOLIA_APP_ID").unwrap();
    let url = var("ALGOLIA_API_URL").unwrap();

    let url = format!("{}/1/indexes/{}/batch", url, index);

    let client = Client::new();

    let request = BatchAdd::<Post>::new(post);

    let res = client
        .post(&url)
        .header("X-Algolia-API-Key", api_key)
        .header("X-Algolia-Application-Id", app_id)
        .json(&serde_json::json!(request))
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let res = res.json::<UnsuccessfulResponse>().await?;

        eprintln!("Failed to add objects: {}", res.message);

        return Err(ClientError {
            message: res.message,
            url: Some(url),
            status: Some(res.status),
        });
    }

    Ok(())
}

#[allow(dead_code)]
async fn get_indexes() -> Result<Vec<String>, ClientError> {
    let api_key = var("ALGOLIA_API_KEY").unwrap();
    let app_id = var("ALGOLIA_APP_ID").unwrap();
    let url = var("ALGOLIA_API_URL").unwrap();

    let url = format!("{}/1/indexes", url);

    let client = Client::new();

    let res = client
        .get(&url)
        .header("X-Algolia-API-Key", api_key)
        .header("X-Algolia-Application-Id", app_id)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let res = res.json::<UnsuccessfulResponse>().await?;

        eprintln!("Failed to get indexes: {}", res.message);

        return Err(ClientError {
            message: res.message,
            url: Some(url),
            status: Some(res.status),
        });
    }

    let res = res.json::<ListIndexes>().await?;

    Ok(res.items.into_iter().map(|index| index.name).collect())
}

#[allow(dead_code)]
async fn is_index_exist(index: &str) -> Result<bool, ClientError> {
    let indexes = get_indexes().await?;

    Ok(indexes.contains(&index.to_string()))
}

async fn setting_index(index: &str) -> Result<(), ClientError> {
    let api_key = var("ALGOLIA_API_KEY").unwrap();
    let app_id = var("ALGOLIA_APP_ID").unwrap();
    let url = var("ALGOLIA_API_URL").unwrap();

    let url = format!(
        "{}/1/indexes/{}/settings?forwardToReplicas=true",
        url, index
    );

    let client = reqwest::Client::new();

    let index_setting = IndexSetting {
        hits_per_page: 10,
        searchable_attributes: vec![
            "title".to_string(),
            // "slug".to_string(),
            "text".to_string(),
            // "categories".to_string(),
            // "tags".to_string(),
        ],
        attributes_to_snippet: vec![format!("{}:250", "text")],
        attributes_to_highlight: vec!["title".to_string()],
        custom_ranking: vec!["desc(created)".to_string()],
    };

    let res = client
        .put(&url)
        .header("X-Algolia-API-Key", api_key)
        .header("X-Algolia-Application-Id", app_id)
        .json(&serde_json::json!(index_setting))
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let res = res.json::<UnsuccessfulResponse>().await?;

        eprintln!("Failed to create index: {}", res.message);

        return Err(ClientError {
            message: res.message,
            url: Some(url),
            status: Some(res.status),
        });
    }

    Ok(())
}

pub async fn sync(index: &str, posts: &Vec<Post>) -> Result<(), ClientError> {
    add_abjects(index, posts).await?;

    setting_index(index).await?;

    Ok(())
}
