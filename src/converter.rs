use crate::model::Config;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;

pub struct Converter {
    pub config: Config,
}

impl Converter {
    pub fn build(input_path: &String) -> anyhow::Result<Self> {
        let zip_file = Self::find_zip_file(input_path)?;

        let config_path = Self::extract_config_file(&zip_file)?;
        let reader = fs::File::open(config_path)?;
        let config = serde_yaml::from_reader(reader)?;
        Ok(Self { config })
    }

    pub fn convert(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn find_zip_file(input_path: &String) -> anyhow::Result<String> {
        let dir = Path::new(input_path);
        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "zip" {
                    return Ok(path.to_string_lossy().into_owned());
                }
            }
        }

        Err(anyhow::anyhow!("No .zip file found in the directory"))
    }

    fn extract_config_file(zip_file: &str) -> anyhow::Result<PathBuf> {
        let file = fs::File::open(zip_file)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_owned();
            if file_name == "testdata/config.yaml" {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let config_path = PathBuf::from("/home/serein/Projects/yarusto/src/config.yaml");
                fs::write(&config_path, content)?;
                return Ok(config_path);
            }
        }

        Err(anyhow::anyhow!(
            "testdata/config.yaml not found in the zip file"
        ))
    }
}
