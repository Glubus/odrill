use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateManifest {
    pub template: TemplateConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
}
