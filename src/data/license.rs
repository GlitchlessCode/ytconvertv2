use super::*;

use nanoserde::DeJson;

#[derive(Debug, Data, Clone, DeJson)]
pub struct License {
    pub name: String,
    pub version: Option<String>,
    pub authors: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub description: Option<String>,
}
