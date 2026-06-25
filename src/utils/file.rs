use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;

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
            "com",         // organization qualifier
            "hayate_sato", // organization name
            "phantom_ta",  // application name
        )
        .expect("Failed to locate application data directory");

        project_dirs.data_local_dir().join(relative)
    }
}

pub trait SaveLoad: Sized + Default {
    const PATH: &'static str;
    fn load_default_path() -> anyhow::Result<Self> {
        Self::load(Self::PATH)
    }
    fn save_default_path(&self) -> anyhow::Result<()> {
        self.save(Self::PATH)
    }
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;

    fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

pub trait RonSaveLoad: Sized + Default + serde::Serialize + serde::de::DeserializeOwned {
    const PATH: &'static str;
    fn load_ron<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = resolve_path(path);

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let default = Self::default();
            default.save_ron(&path)?;

            return Ok(default);
        }

        let text =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;

        let value =
            ron::from_str(&text).with_context(|| format!("Failed to deserialize {:?}", path))?;

        Ok(value)
    }

    fn save_ron<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = resolve_path(path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;

        fs::write(&path, text).with_context(|| format!("Failed to write {:?}", path))?;

        Ok(())
    }
}

/// RonSaveLoadを実装している型は
/// SaveLoadを自動でもらえる
impl<T> SaveLoad for T
where
    T: RonSaveLoad,
{
    const PATH: &'static str = <T as RonSaveLoad>::PATH;
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Self::load_ron(path)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        self.save_ron(path)
    }
}
