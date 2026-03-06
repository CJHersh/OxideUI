//! Figma REST API client

use crate::figma::types::VariablesResponse;
use thiserror::Error;

const FIGMA_API_BASE: &str = "https://api.figma.com/v1";

#[derive(Error, Debug)]
pub enum FigmaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Client for Figma REST API
pub struct FigmaClient {
    client: reqwest::Client,
    token: String,
}

impl FigmaClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.into(),
        }
    }

    /// Get file metadata
    #[allow(dead_code)]
    pub async fn get_file(&self, file_key: &str) -> Result<serde_json::Value, FigmaError> {
        let url = format!("{}/files/{}", FIGMA_API_BASE, file_key);
        let res = self
            .client
            .get(&url)
            .header("X-Figma-Token", &self.token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(FigmaError::Api(format!("{}: {}", status, body)));
        }

        let json: serde_json::Value = res.json().await?;
        Ok(json)
    }

    /// Get file metadata (lightweight)
    #[allow(dead_code)]
    pub async fn get_file_meta(&self, file_key: &str) -> Result<serde_json::Value, FigmaError> {
        let url = format!("{}/files/{}/meta", FIGMA_API_BASE, file_key);
        let res = self
            .client
            .get(&url)
            .header("X-Figma-Token", &self.token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(FigmaError::Api(format!("{}: {}", status, body)));
        }

        let json: serde_json::Value = res.json().await?;
        Ok(json)
    }

    /// Get local variables from a file
    /// Note: Figma Variables API may use a different endpoint; this implements the expected structure
    pub async fn get_local_variables(
        &self,
        file_key: &str,
    ) -> Result<VariablesResponse, FigmaError> {
        // Try the variables endpoint - Figma REST API for variables
        let url = format!("{}/files/{}/variables/local", FIGMA_API_BASE, file_key);
        let res = self
            .client
            .get(&url)
            .header("X-Figma-Token", &self.token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(FigmaError::Api(format!("{}: {}", status, body)));
        }

        let json: serde_json::Value = res.json().await?;

        // If the API returns a different structure, try to parse as VariablesResponse
        serde_json::from_value(json).map_err(|e| FigmaError::Parse(e.to_string()))
    }
}
