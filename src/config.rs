// TODO - Add theme config

use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

#[cfg(not(windows))]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

#[cfg(windows)]
use std::{
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
};

use nanoserde::{DeBin, SerBin};
use platform_dirs::AppDirs;
use vizia::prelude::*;

#[cfg(windows)]
#[derive(Debug, SerBin, DeBin)]
pub struct SerializablePath {
    path: Vec<u16>,
}

#[cfg(not(windows))]
#[derive(Debug, SerBin, DeBin)]
pub struct SerializablePath {
    path: Vec<u8>,
}

#[cfg(not(windows))]
impl From<&Option<PathBuf>> for SerializablePath {
    fn from(value: &Option<PathBuf>) -> Self {
        let path: Vec<u8> = match value {
            Some(path) => path
                .as_os_str()
                .as_bytes()
                .iter()
                .map(|uint| uint.to_owned())
                .collect(),
            None => vec![],
        };

        Self { path }
    }
}

#[cfg(not(windows))]
impl From<&SerializablePath> for Option<PathBuf> {
    fn from(value: &SerializablePath) -> Self {
        if value.path.len() == 0 {
            None
        } else {
            Some(PathBuf::from(OsStr::from_bytes(&value.path)))
        }
    }
}

#[cfg(windows)]
impl From<&Option<PathBuf>> for SerializablePath {
    fn from(value: &Option<PathBuf>) -> Self {
        let path: Vec<u16> = match value {
            Some(path) => path
                .as_os_str()
                .encode_wide()
                .into_iter()
                // .map(|uint| uint.to_owned())
                .collect(),
            None => vec![],
        };

        Self { path }
    }
}

#[cfg(windows)]
impl From<&SerializablePath> for Option<PathBuf> {
    fn from(value: &SerializablePath) -> Self {
        if value.path.len() == 0 {
            None
        } else {
            Some(PathBuf::from(OsString::from_wide(&value.path)))
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
struct Config {
    #[nserde(proxy = "SerializablePath")]
    location: Option<PathBuf>,
}

impl Config {
    fn load(mut config_path: PathBuf) -> Option<Self> {
        if let Err(error) = std::fs::create_dir_all(config_path.clone()) {
            eprintln!("Failed to create parent directories due to error: {error}")
        }

        config_path.push("config.bin");

        if !(config_path.exists()) {
            return Some(Self::default());
        }

        match create_and_read(config_path) {
            // Got file contents
            Ok(file) => {
                match Self::deserialize_bin(&file) {
                    // Got config
                    Ok(config) => Some(config),

                    // Got error
                    Err(error) => {
                        eprintln!("Could not deserialize config due to error: {error}");
                        None
                    }
                }
            }

            // Got Error
            Err(error) => {
                eprintln!("Could not open config due to error: {error}");
                None
            }
        }
    }

    fn save(&self, mut config_path: PathBuf) {
        if let Err(error) = std::fs::create_dir_all(config_path.clone()) {
            eprintln!("Failed to create parent directories due to error: {error}")
        }

        config_path.push("config.bin");

        match std::fs::File::create(config_path) {
            // Got file
            Ok(mut file) => {
                let serialized = self.serialize_bin();
                if let Err(error) = file.write_all(&serialized) {
                    eprintln!("Could not deserialize config due to error: {error}");
                }
            }

            // Got Error
            Err(error) => {
                eprintln!("Could not open config due to error: {error}");
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { location: None }
    }
}

#[derive(Debug)]
pub struct ConfigModel {
    path: Option<PathBuf>,
    inner: Option<Config>,
}

impl ConfigModel {
    pub fn open_config_path(dirs: Option<AppDirs>) -> Self {
        match dirs {
            None => Self {
                path: None,
                inner: None,
            },
            Some(AppDirs { config_dir, .. }) => Self {
                path: Some(config_dir.clone()),
                inner: Config::load(config_dir),
            },
        }
    }

    fn has_path(&self) -> bool {
        self.path.is_some()
    }

    fn config(&mut self) -> &mut Config {
        if !(self.has_path()) {
            panic!("Cannot create a config when missing a config path");
        }

        match self.inner {
            Some(ref mut config) => config,
            None => {
                self.inner = Some(Config::default());
                self.inner
                    .as_mut()
                    .expect("should unwrap config, value set above")
            }
        }
    }

    fn save(&mut self) {
        if let Some(ref config_path) = self.path {
            // Only save if it exists
            if let Some(ref config) = self.inner {
                config.save(config_path.to_owned());
            }
        } else {
            panic!("Cannot save config when missing a config path");
        }
    }
}

impl Model for ConfigModel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _meta| {
            if !(self.has_path()) {
                return;
            }

            match event {
                ConfigEvent::SetExportPath(path) => {
                    if path.exists() {
                        self.config().location = Some(path.to_owned());
                    } else {
                        self.config().location = None;
                    }
                }

                ConfigEvent::RequestSetup => {
                    if let Some(ref config) = self.inner {
                        cx.emit(ConfigEvent::ConfigSetup {
                            location: config.location.to_owned(),
                        });
                    }
                }
                _ => (),
            }

            self.save()
        })
    }
}

#[non_exhaustive]
pub enum ConfigEvent {
    RequestSetup,
    ConfigSetup { location: Option<PathBuf> },

    SetExportPath(PathBuf),
}

fn create_and_read<P>(path: P) -> std::io::Result<Vec<u8>>
where
    P: AsRef<std::path::Path>,
{
    let mut buf = Vec::new();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?
        .read_to_end(&mut buf)?;
    Ok(buf)
}
