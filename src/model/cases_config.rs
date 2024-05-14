use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use super::{
    config::Config,
    types::judge::{JudgeType, ResourceLimits, TaskType},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CasesConfig {
    pub score: NonZeroU32,
    pub judge: JudgeType,
    pub resource_limits: ResourceLimits,
    pub task: TaskType,
}

impl<'a> TryFrom<Box<dyn Config + 'a>> for CasesConfig {
    type Error = crate::error::Error;

    fn try_from(value: Box<dyn Config + 'a>) -> Result<CasesConfig, Self::Error> {
        Ok(CasesConfig {
            score: value.score()?,
            judge: value.judge()?,
            resource_limits: value.resource_limits()?,
            task: value.task()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use serde_json::json;

    use crate::model::types::judge::Case;

    use super::*;

    fn non_zero(value: u32) -> NonZeroU32 {
        debug_assert_ne!(value, 0);
        NonZeroU32::new(value).unwrap()
    }

    #[test]
    fn serialize_test() {
        serde_json::to_string_pretty(&CasesConfig {
            score: non_zero(100),
            judge: JudgeType::Classic,
            resource_limits: ResourceLimits {
                time: 1000,
                memory: 512,
            },
            task: TaskType::Simple {
                cases: vec![
                    Case {
                        input: "1.in".into(),
                        answer: "1.ans".into(),
                        score: None,
                    },
                    Case {
                        input: "2.in".into(),
                        answer: "2.ans".into(),
                        score: NonZeroU32::new(60),
                    },
                ],
            },
        })
        .unwrap();
    }

    #[test]
    fn deserialize_test() {
        serde_json::from_value::<CasesConfig>(json!({
          "score": 100,
          "judge": {
            "judgeType": "classic"
          },
          "resourceLimits": {
            "time": 1000,
            "memory": 256
          },
          "task": {
            "taskType": "simple",
            "cases": [
              {
                "input": "1.in",
                "answer": "1.ans"
              },
              {
                "input": "2.in",
                "answer": "2.ans",
                "score": 60
              }
            ]
          }
        }))
        .unwrap();

        serde_json::from_value::<CasesConfig>(json!({
          "score": 100,
          "judge": {
            "judgeType": "special-judge",
            "checker": "checker.cpp"
          },
          "resourceLimits": {
            "time": 1000,
            "memory": 256
          },
          "task": {
            "taskType": "subtask",
            "subtasks": [
              {
                "cases": [
                  {
                    "input": "1.in",
                    "answer": "1.ans"
                  },
                  {
                    "input": "2.in",
                    "answer": "2.ans"
                  }
                ],
                "score": 40
              },
              {
                "cases": [
                  {
                    "input": "3.in",
                    "answer": "3.ans"
                  },
                  {
                    "input": "4.in",
                    "answer": "4.ans"
                  }
                ],
                "score": 60
              }
            ]
          }
        }))
        .unwrap();
    }
}
