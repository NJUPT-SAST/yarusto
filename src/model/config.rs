use crate::model::types::judge::JudgeType;
use crate::model::types::judge::TaskType;
use std::num::NonZeroU32;

use super::types::judge::ResourceLimits;

use crate::error::Result;

pub trait Config {
    fn score(&self) -> Result<NonZeroU32> {
        Ok(NonZeroU32::new(100).unwrap())
    }

    fn judge(&self) -> Result<JudgeType> {
        Ok(JudgeType::Classic)
    }

    fn resource_limits(&self) -> Result<ResourceLimits> {
        Ok(ResourceLimits {
            time: 1000,
            memory: 256,
        })
    }

    fn task(&self) -> Result<TaskType>;
}
