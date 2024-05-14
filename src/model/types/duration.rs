use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize,
};

#[derive(Debug)]
pub struct Duration {
    value: u32,
    unit: DurationUnit,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            value: 1000,
            unit: DurationUnit::Milliseconds,
        }
    }
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing duration")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Some((value, unit)) = value.split_once(|c: char| c.is_ascii_alphabetic()) {
            let value = value.parse().expect("Invalid duration");
            let unit = DurationUnit::from_str(unit).unwrap_or(DurationUnit::Milliseconds);

            Ok(Duration { value, unit })
        } else {
            Err(de::Error::invalid_value(
                serde::de::Unexpected::Str(value),
                &self,
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DurationVisitor)
    }
}

impl Duration {
    pub fn as_millis(&self) -> u32 {
        match self.unit {
            DurationUnit::Milliseconds => self.value,
            DurationUnit::Seconds => self.value * 1000,
        }
    }
}

#[derive(Debug)]
pub enum DurationUnit {
    Milliseconds,
    Seconds,
}

impl DurationUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ms" => Some(Self::Milliseconds),
            "s" => Some(Self::Seconds),
            _ => None,
        }
    }
}
