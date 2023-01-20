use thiserror::Error;

pub type PixResult<T> = anyhow::Result<T, anyhow::Error>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// Graphics renderer error.
    #[error("renderer error: {0}")]
    Renderer(String),
}