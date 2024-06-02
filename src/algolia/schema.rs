use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Index {
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub entries: u32,
    #[serde(rename = "dataSize")]
    pub data_size: u32,
    #[serde(rename = "fileSize")]
    pub file_size: u32,
    #[serde(rename = "lastBuildTimeS")]
    pub last_build_time_s: u32,
    #[serde(rename = "numberOfPendingTask")]
    pub number_of_pending_task: u32,
    #[serde(rename = "pendingTask")]
    pub pending_task: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListIndexes {
    pub items: Vec<Index>,
    #[serde(rename = "nbPages")]
    pub nb_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct AddObject<T>
where
    T: Clone,
{
    action: String,
    body: T,
}

#[derive(Debug, Serialize)]
pub struct BatchAdd<T>
where
    T: Clone,
{
    pub requests: Vec<AddObject<T>>,
}

impl<T> BatchAdd<T>
where
    T: Clone,
{
    pub fn new(data: &Vec<T>) -> Self {
        Self {
            requests: data
                .into_iter()
                .map(|d| AddObject {
                    action: "addObject".to_string(),
                    body: d.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UnsuccessfulResponse {
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Serialize)]
pub struct IndexSetting {
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: u32,
    #[serde(rename = "searchableAttributes")]
    pub searchable_attributes: Vec<String>,
    #[serde(rename = "attributesToSnippet")]
    pub attributes_to_snippet: Vec<String>,
    #[serde(rename = "attributesToHighlight")]
    pub attributes_to_highlight: Vec<String>,
    #[serde(rename = "customRanking")]
    pub custom_ranking: Vec<String>,
}
