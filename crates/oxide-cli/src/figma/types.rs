#![allow(non_snake_case)]
//! Figma API response types for Variables

use serde::Deserialize;
use std::collections::HashMap;

/// Root response from Figma Variables API
#[derive(Debug, Deserialize)]
pub struct VariablesResponse {
    pub meta: Option<VariablesMeta>,
    #[serde(default)]
    pub variableCollections: Vec<VariableCollection>,
}

/// Metadata wrapper returned by `/v1/files/:key/variables/local`
#[derive(Debug, Deserialize)]
pub struct VariablesMeta {
    #[serde(default)]
    pub variableCollections: HashMap<String, VariableCollection>,
    #[serde(default)]
    pub variables: HashMap<String, Variable>,
}

/// A collection of variables (e.g. "primitives", "semantic")
#[derive(Debug, Deserialize)]
pub struct VariableCollection {
    pub name: String,
    #[serde(default)]
    pub modes: Vec<Mode>,
    #[serde(default)]
    pub variableIds: Vec<String>,
}

/// A mode within a collection (e.g. "light", "dark")
#[derive(Debug, Deserialize)]
pub struct Mode {
    pub modeId: String,
    pub name: String,
}

/// Variable definition
#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub resolvedType: String,
    #[serde(default)]
    pub valuesByMode: serde_json::Map<String, serde_json::Value>,
}
