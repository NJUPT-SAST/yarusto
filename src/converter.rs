use crate::model::CasesConfig;
use async_walkdir::WalkDir;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use tokio_stream::StreamExt;
use zip::ZipArchive;

pub struct Converter {
    pub config_paths: Vec<PathBuf>,
}

const TMP_DIR: &str = "./tmp";

impl Converter {
    pub async fn build(input_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let zip_file = find_zip_file(input_path)?;

        let config_paths = extract_config_file(&zip_file).await?;

        Ok(Self { config_paths })
    }

    pub async fn rename(&self) -> anyhow::Result<&Self> {
        let regex = Regex::new(r"^.*(\d)\.(in|ans)$").unwrap();
        let mut entries = WalkDir::new(TMP_DIR);
        while let Some(entry) = entries.try_next().await? {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            regex.captures(filename).and_then(|cap| {
                let digit = cap.get(1)?.as_str();
                let new_path =
                    path.with_file_name(format!("{}.{}", digit, path.extension()?.to_str()?));
                fs::rename(&path, new_path).ok()
            });
        }
        Ok(self)
    }

    pub async fn convert(&self) -> anyhow::Result<&Self> {
        for config_path in self.config_paths.clone() {
            tokio::spawn(async move {
                let reader = fs::File::open(&config_path).unwrap();
                let config: CasesConfig = serde_yaml::from_reader(reader).unwrap();

                let parent_dir = config_path.parent().unwrap();
                let toml_path = parent_dir.join("config.toml");
                fs::File::create(&toml_path).unwrap();

                let toml_string = toml::to_string(&config).unwrap();
                fs::write(&toml_path, toml_string).unwrap();
            })
            .await?;
        }

        Ok(self)
    }

    pub fn tar(&self, output_path: impl AsRef<Path>) -> anyhow::Result<&Self> {
        let tar_file = output_path.as_ref().join("config.tar.zstd");
        let file = fs::File::create(tar_file)?;
        let encoder = zstd::stream::Encoder::new(file, 0)?;

        let mut tar_builder = tar::Builder::new(encoder);
        tar_builder.append_dir_all(&output_path, TMP_DIR)?;
        tar_builder.finish()?;

        Ok(self)
    }

    pub fn cleanup(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(TMP_DIR)?;
        Ok(())
    }
}

fn find_zip_file(input_path: impl AsRef<Path>) -> anyhow::Result<String> {
    let entries = fs::read_dir(input_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "zip" {
                return Ok(path.to_string_lossy().into_owned());
            }
        }
    }

    anyhow::bail!("No .zip file found in the directory")
}

async fn extract_config_file(zip_file: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_file)?;
    ZipArchive::new(file)?.extract(TMP_DIR)?;

    let mut yaml_files = Vec::new();

    let mut entries = WalkDir::new(TMP_DIR);
    while let Some(entry) = entries.try_next().await? {
        let path = entry.path();
        if path.file_name() == Some(OsStr::new("config.yaml"))
            || path.file_name() == Some(OsStr::new("config.yml"))
        {
            yaml_files.push(path);
        }
    }

    eprintln!("Found {} YAML files", yaml_files.len());
    eprintln!("{:?}", yaml_files);

    Ok(yaml_files)
}
