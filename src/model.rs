use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub score: i32,
    pub judge: Judge,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Judge {
    Interactive(InteractiveMode),
    Simple(SimpleMode),
    Special(SpecialMode),
    SubTask(SubTaskMode),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub time: i32,
    pub memory: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveMode {
    pub judge_type: String,
    pub interactor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleMode {
    pub judge_type: String,
    pub task_type: String,
    pub cases: Vec<Case>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialMode {
    pub judge_type: String,
    pub checker: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTaskMode {
    pub judge_type: String,
    pub task_type: String,
    pub subtasks: Vec<SubTask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub input: String,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTask {
    pub score: i32,
    pub cases: Vec<Case>,
}
