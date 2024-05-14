use std::result::Result as StdResult;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    #[error("Invalid memory size: {0}")]
    InvalidMemorySize(String),
    #[error("Invalid judge type: {0}")]
    InvalidJudgeType(String),
    #[error("Invalid task type: {0}")]
    InvalidTaskType(String),
    #[error("Invalid resource limits: {0}")]
    InvalidResourceLimits(String),
    #[error("Invalid filename: {0}")]
    InvalidFilename(#[from] core::convert::Infallible),
    #[error("Invalid score: {0}")]
    InvalidScore(u32),
}

pub type Result<T, E = Error> = StdResult<T, E>;
