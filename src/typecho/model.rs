use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub cid: u32,
    pub title: String,
    pub slug: String,
    pub text: String,
    pub created: u32,
    pub modified: u32,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    #[serde(rename = "objectID")]
    pub object_id: String,
}
