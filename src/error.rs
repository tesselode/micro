use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("{0}")]
pub struct GlError(pub String);
