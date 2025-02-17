use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WhoAmIResponse {
    pub total_requests: u64,
    pub ipaddress: String,
    pub language: Option<String>,
    pub software: Option<String>,
    pub os: Option<String>,
    pub browser_name: Option<String>,
    pub browser_version: Option<String>,
    pub parsed_language: Option<String>,
}
