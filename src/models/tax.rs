use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateTaxReportsParams {
    pub start_time: i64,
    pub end_time: i64,
    pub items: Vec<TaxExportItem>,
    #[serde(default)]
    pub source_institution: Option<String>,
    #[serde(default)]
    pub export_file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxExportItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub number: String,
}

pub type BatchCreateTaxReportsResponse = Option<serde_json::Value>;

pub type BatchQueryTaxReportsResponse = Option<serde_json::Value>;
