use serde;
use serde_json;
use xdg;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(serde::Deserialize, Debug)]
pub struct ConfigJson {
    pub folders_to_prune:   Vec<String>,
    pub files_to_prune:     Vec<String>,
}

#[derive(Debug)]
pub struct AppConfig {
    app_name: String,
    pub config: ConfigJson,
}

static DEFAULT_CONFIG_JSON: &str = r#"{
    "folders_to_prune": [".svn", ".git", ".hg"],
    "files_to_prune":   ["*~"]
}
"#;

impl AppConfig {
    pub fn new(app_name: &str) -> Result<AppConfig, String> {
        let config_path = config_file_path(&app_name);

        let mut config_data = String::new();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(data) => { config_data.push_str(&data) },
                Err(e) => { return Err(format!("Error reading {} - {}", &config_path.display(), e.to_string())) }
            };
        } else {
            config_data.push_str(DEFAULT_CONFIG_JSON);
        };

        let app_config = AppConfig {
            app_name: app_name.to_string(),
            config: match serde_json::from_str(&config_data) {
                Ok(config) => config,
                Err(e) => { return Err(format!("Error parsing config {} - {}", &config_path.display(), e.to_string())) }
            }
        };
        Ok(app_config)
    }

    pub fn config_file_path(&self) -> PathBuf {
        config_file_path(&self.app_name)
    }


    pub fn save_default_config(&self) -> std::io::Result<()> {
        let config_path = self.config_file_path();

        if config_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, "config file already exists"));
        }

        let mut f = File::create(config_path)?;
        f.write_all(DEFAULT_CONFIG_JSON.as_bytes())?;
        Ok(())
    }
}

fn config_file_path(app_name: &str) -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::new().unwrap();

    PathBuf::from(xdg_dirs.place_config_file(format!("{}.json", &app_name)).unwrap())
}

#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}
