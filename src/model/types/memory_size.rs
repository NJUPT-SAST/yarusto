use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug)]
pub struct MemorySize {
    value: u32,
    unit: MemorySizeUnit,
}

impl MemorySize {
    pub fn as_kib(&self) -> u32 {
        match self.unit {
            MemorySizeUnit::KiB => self.value,
            MemorySizeUnit::MiB => self.value * 1024,
            MemorySizeUnit::GiB => self.value * 1024 * 1024,
        }
    }
}

impl Default for MemorySize {
    fn default() -> Self {
        Self {
            value: 128,
            unit: MemorySizeUnit::MiB,
        }
    }
}

#[derive(Debug)]
pub enum MemorySizeUnit {
    KiB,
    MiB,
    GiB,
}

impl MemorySizeUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "KiB" | "KB" | "kb" => Some(Self::KiB),
            "MiB" | "MB" | "mb" => Some(Self::MiB),
            "GiB" | "GB" | "gb" => Some(Self::GiB),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for MemorySize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MemorySizeVisitor;

        impl<'de> Visitor<'de> for MemorySizeVisitor {
            type Value = MemorySize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing memory size")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let (value, unit) = value.split_at(
                    value
                        .find(|c: char| c.is_ascii_alphabetic())
                        .unwrap_or(value.len()),
                );
                let value = value.parse().map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(value), &"a valid integer")
                })?;
                let unit = MemorySizeUnit::from_str(unit).unwrap_or(MemorySizeUnit::MiB);

                Ok(MemorySize { value, unit })
            }
        }

        deserializer.deserialize_str(MemorySizeVisitor)
    }
}

impl Serialize for MemorySize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.unit {
            MemorySizeUnit::KiB => serializer.serialize_u32(self.value),
            MemorySizeUnit::MiB => serializer.serialize_u32(self.value * 1024),
            MemorySizeUnit::GiB => serializer.serialize_u32(self.value * 1024 * 1024),
        }
    }
}
