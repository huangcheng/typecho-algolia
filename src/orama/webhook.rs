use crate::ClientError;

async fn snapshot(
    webhook_url: &str,
    secret_key: &str,
    index_id: &str,
    posts: &str,
) -> Result<(), ClientError> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}/snapshot", webhook_url, index_id);

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", secret_key))
        .body(posts.to_string())
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Failed to submit snapshot");

        return Err(ClientError {
            message: "Failed to submit snapshot".to_string(),
            url: Some(url.clone()),
            status: Some(res.status().as_u16()),
        });
    }

    Ok(())
}

async fn deploy(webhook_url: &str, secret_key: &str, index_id: &str) -> Result<(), ClientError> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}/deploy", webhook_url, index_id);

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", secret_key))
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Failed to deploy");

        return Err(ClientError {
            message: "Failed to deploy".to_string(),
            url: Some(url.clone()),
            status: Some(res.status().as_u16()),
        });
    }

    Ok(())
}

pub async fn start_snapshot(
    webhook_url: &str,
    secret_key: &str,
    index_id: &str,
    posts: &str,
) -> Result<(), ClientError> {
    snapshot(webhook_url, secret_key, index_id, posts).await?;

    deploy(webhook_url, secret_key, index_id).await
}
