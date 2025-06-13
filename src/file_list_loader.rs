
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileSourceItem {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}