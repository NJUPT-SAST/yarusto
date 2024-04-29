use async_walkdir::WalkDir;

use indicatif::{style, ProgressBar, ProgressIterator};
use serde::{Deserialize, Serialize};

use std::path::Path;
use std::{ffi::OsStr, path::PathBuf};
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio_stream::StreamExt;

pub struct Converter {
    yaml_files: Vec<PathBuf>,
    toml_results: Vec<String>,
    toml_files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    name: String,
    age: u8,
    city: String,
}

impl Converter {
    pub async fn build(path: &String) -> anyhow::Result<Self> {
        let mut entries = WalkDir::new(&path);
        let mut yaml_files = Vec::new();
        let mut toml_files = Vec::new();

        while let Some(entry) = entries.try_next().await? {
            if let Some("yaml" | "yml") =
                entry.path().extension().unwrap_or(OsStr::new("")).to_str()
            {
                let file_name = entry.file_name().to_string_lossy().to_string();
                yaml_files.push(entry.path());
                toml_files.push(
                    file_name
                        .trim_end_matches(".yaml")
                        .trim_end_matches(".yml")
                        .to_string()
                        + ".toml",
                );
            }
        }

        eprintln!("Found {} YAML files", yaml_files.len());
        eprintln!("{:?}", yaml_files);

        let toml_results = Vec::new();

        Ok(Converter {
            yaml_files,
            toml_results,
            toml_files,
        })
    }

    pub async fn process(&mut self) -> anyhow::Result<&mut Self> {
        let pb = ProgressBar::new(self.yaml_files.len() as u64);
        pb.set_style(
            style::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .map_err(|e| anyhow::anyhow!("Failed to set progress bar style: {:?}", e))?,
        );
        for file in self.yaml_files.iter().progress_with(pb) {
            let file = File::open(file).await?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).await?;
            let data: Data = serde_yaml::from_slice(&buffer)?;
            let toml = toml::to_string(&data)?;
            self.toml_results.push(toml);
        }
        Ok(self)
    }

    pub async fn save(&mut self, output: &String) -> anyhow::Result<&mut Self> {
        if !Path::new(&output).exists() {
            create_dir_all(&output).await?;
        }
        for (toml, file_name) in self.toml_results.iter().zip(self.toml_files.iter()) {
            let file_path = format!("{}/{}", output, file_name);
            let mut file = File::create(file_path).await?;
            file.write_all(toml.as_bytes()).await?;
        }
        Ok(self)
    }
}
