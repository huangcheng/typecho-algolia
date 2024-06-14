pub mod algolia;
pub mod orama;
pub mod typecho;

#[derive(Debug)]
pub struct ClientError {
    pub message: String,
    pub url: Option<String>,
    pub status: Option<u16>,
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            message: err.to_string(),
            url: err.url().map(|url| url.as_str().to_string()),
            status: err.status().map(|status| status.as_u16()),
        }
    }
}
