use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, KaelError>;

#[derive(Debug, thiserror::Error)]
pub enum KaelError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Template error: {0}")]
    Template(#[from] minijinja::Error),

    #[error("PRD error: {message}")]
    Prd { message: String },

    #[error("Project error: {message}")]
    Project { message: String },

    #[error("File already exists: {path}")]
    FileExists { path: PathBuf },

    #[error("Registry resource not found: {name}")]
    RegistryNotFound { name: String },
}
