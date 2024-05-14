use std::{num::NonZeroU32, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "judgeType")]
pub enum JudgeType {
    Classic,
    SpecialJudge { checker: PathBuf },
    Interactive { interactor: PathBuf },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceLimits {
    pub time: u32,   // ms
    pub memory: u32, // MiB
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "taskType")]
pub enum TaskType {
    Simple { cases: Vec<Case> },
    Subtask { subtasks: Vec<Subtask> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Case {
    pub input: PathBuf,
    pub answer: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<NonZeroU32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subtask {
    pub cases: Vec<Case>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<NonZeroU32>,
}
