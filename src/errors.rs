use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, LinAppError>;

#[derive(Debug, Error, Diagnostic)]
pub enum LinAppError {
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error)
}