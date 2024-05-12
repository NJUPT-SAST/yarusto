use crate::model::CasesConfig;
use async_walkdir::WalkDir;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use zip::ZipArchive;

pub struct Converter {
    config_paths: Vec<PathBuf>,
    temp_dir: TempDir,
}

impl Converter {
    pub async fn build(input_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let zip_file = find_zip_file(input_path).await?;

        let temp_dir = TempDir::new("yarusto")?;

        let config_paths = extract_config_file(&zip_file, &temp_dir.path().to_path_buf()).await?;

        Ok(Self {
            config_paths,
            temp_dir,
        })
    }

    pub async fn rename(&self) -> anyhow::Result<&Self> {
        let mut entries = WalkDir::new(self.temp_dir.path());
        while let Some(entry) = entries.try_next().await? {
            let path = entry.path();
            let filename = path
                .file_name()
                .expect("Should have a filename")
                .to_string_lossy();
            let (prefix, ext) = filename.split_once('.').expect("Should have an extension");
            if let "in" | "out" | "ans" = ext {
                let digit: String = prefix
                    .chars()
                    .rev()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect();
                let new_filename = format!("{}.{}", digit, if ext == "out" { "ans" } else { ext });
                fs::rename(&path, new_filename).await?;
            }
        }
        Ok(self)
    }

    pub async fn convert(&self) -> anyhow::Result<&Self> {
        for config_path in self.config_paths.iter() {
            let reader = File::open(&config_path).await?;
            let config: CasesConfig = serde_yaml::from_reader(reader.into_std().await)?;

            let parent_dir = config_path.parent().expect("No parent directory");
            let toml_path = parent_dir.join("config.toml");
            let mut toml_file = File::create(&toml_path).await?;

            toml_file
                .write_all(toml::to_string(&config)?.as_bytes())
                .await?;
        }

        Ok(self)
    }

    pub async fn tar(&self, output_path: impl AsRef<Path>) -> anyhow::Result<&Self> {
        let tar_file = output_path.as_ref().join("config.tar.zst");
        let file = File::create(tar_file).await?;
        let encoder = zstd::Encoder::new(file.into_std().await, 0)?;
        let mut tar_builder = tar::Builder::new(encoder);
        tar_builder.append_dir_all(&output_path, self.temp_dir.path())?;
        tar_builder.finish()?;

        Ok(self)
    }
}

async fn find_zip_file(input_path: impl AsRef<Path>) -> anyhow::Result<String> {
    for mut entry in fs::read_dir(input_path).await.into_iter() {
        let path = entry
            .next_entry()
            .await?
            .expect("Should have an entry")
            .path();
        if let Some(extension) = path.extension() {
            if extension == "zip" {
                return Ok(path.to_string_lossy().into_owned());
            }
        }
    }

    anyhow::bail!("No .zip file found in the directory")
}

async fn extract_config_file(
    zip_file: impl AsRef<Path>,
    temp_dir: &PathBuf,
) -> anyhow::Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_file).await?;
    ZipArchive::new(file.into_std().await)?.extract(&temp_dir)?;

    let mut yaml_files = Vec::new();

    let mut entries = WalkDir::new(&temp_dir);
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
