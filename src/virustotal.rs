use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AVEntry {
    pub category: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AVEntries {
    #[serde(rename = "Symantec")]
    pub symantec: Option<AVEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attributes {
    pub last_analysis_results: AVEntries,
    pub md5: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data {
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirustotalResponse {
    pub data: Data,   
}
