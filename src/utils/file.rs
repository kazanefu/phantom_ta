use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;

#[cfg(not(debug_assertions))]
use directories::ProjectDirs;

/// 相対パスを実際の保存先へ変換
pub fn resolve_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let relative = path.as_ref();

    #[cfg(debug_assertions)]
    {
        relative.to_path_buf()
    }

    #[cfg(not(debug_assertions))]
    {
        let project_dirs = ProjectDirs::from(
            "com",        // organization qualifier
            "kazanefu",   // organization name
            "phantom_ta", // application name
        )
        .expect("Failed to locate application data directory");

        project_dirs.data_local_dir().join(relative)
    }
}

pub struct Ron;
pub struct Toml;

pub trait SaveFormat {
    fn serialize<T: serde::Serialize>(value: &T) -> anyhow::Result<String>;
    fn deserialize<T: serde::de::DeserializeOwned>(text: &str) -> anyhow::Result<T>;
}

impl SaveFormat for Ron {
    fn serialize<T: serde::Serialize>(value: &T) -> anyhow::Result<String> {
        Ok(ron::ser::to_string_pretty(
            value,
            ron::ser::PrettyConfig::default(),
        )?)
    }

    fn deserialize<T: serde::de::DeserializeOwned>(text: &str) -> anyhow::Result<T> {
        Ok(ron::from_str(text)?)
    }
}

impl SaveFormat for Toml {
    fn serialize<T: serde::Serialize>(value: &T) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(value)?)
    }

    fn deserialize<T: serde::de::DeserializeOwned>(text: &str) -> anyhow::Result<T> {
        Ok(toml::from_str(text)?)
    }
}

fn load_with_format<T, F>(path: &Path) -> anyhow::Result<T>
where
    T: Default + serde::Serialize + serde::de::DeserializeOwned,
    F: SaveFormat,
{
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let default = T::default();
        save_with_format::<T, F>(&default, path)?;
        return Ok(default);
    }

    let text = fs::read_to_string(path).with_context(|| format!("Failed to read {:?}", path))?;
    F::deserialize(&text).with_context(|| format!("Failed to deserialize {:?}", path))
}

fn save_with_format<T, F>(value: &T, path: &Path) -> anyhow::Result<()>
where
    T: serde::Serialize,
    F: SaveFormat,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let text = F::serialize(value)?;
    fs::write(path, text).with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}

pub trait SaveLoad: Sized + Default + serde::Serialize + serde::de::DeserializeOwned {
    const PATH: &'static str;
    type Format: SaveFormat;
    const USE_APPLICATION_DATA_DIR: bool = true;

    fn load_default_path() -> anyhow::Result<Self> {
        Self::load(Self::PATH)
    }

    fn save_default_path(&self) -> anyhow::Result<()> {
        self.save(Self::PATH)
    }

    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = if Self::USE_APPLICATION_DATA_DIR {
            resolve_path(path)
        } else {
            path.as_ref().to_path_buf()
        };
        load_with_format::<Self, Self::Format>(&path)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = if Self::USE_APPLICATION_DATA_DIR {
            resolve_path(path)
        } else {
            path.as_ref().to_path_buf()
        };
        save_with_format::<Self, Self::Format>(self, &path)
    }
}
