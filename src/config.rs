use crate::config::ConfigError::UnableToOpenFile;
use configparser::ini::Ini;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum ConfigError {
    KeyNotFound(&'static str),
    UnableToDecode(&'static str),
    UnableToOpenFile(String),
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::KeyNotFound(err_name) => write!(f, "{}", err_name),
            ConfigError::UnableToDecode(err_name) => write!(f, "{}", err_name),
            ConfigError::UnableToOpenFile(err_name) => write!(f, "{}", err_name),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::KeyNotFound(err_name) => write!(f, "{}", err_name),
            ConfigError::UnableToDecode(err_name) => write!(f, "{}", err_name),
            ConfigError::UnableToOpenFile(err_name) => write!(f, "{}", err_name),
        }
    }
}

impl Error for ConfigError {}

pub struct Host {
    pub hostname: String,
    pub port: u16,
}

pub struct Config {
    pub version: i32,
    pub host: Host,
}

impl Config {
    pub fn new_from_ini_file(file_path: String) -> Result<Self, ConfigError> {
        let mut config = Ini::new();
        config
            .load(file_path.clone())
            .or(Err(UnableToOpenFile(file_path.clone())))?;
        let version = config
            .get("main", "version")
            .ok_or(ConfigError::KeyNotFound("main: version"))?;
        let version = version
            .parse::<i32>()
            .or(Err(ConfigError::UnableToDecode("main: version")))?;

        Ok(Self {
            version,
            host: Host::new_from_ini_file(file_path)?,
        })
    }
}

impl Host {
    fn new_from_ini_file(file_path: String) -> Result<Self, ConfigError> {
        let mut config = Ini::new();
        config
            .load(file_path.clone())
            .or(Err(UnableToOpenFile(file_path)))?;
        let hostname = config
            .get("host", "hostname")
            .ok_or(ConfigError::KeyNotFound("host: hostname"))?;
        let port = config
            .get("host", "port")
            .ok_or(ConfigError::KeyNotFound("host: port"))?;
        let port = port
            .parse::<u16>()
            .or(Err(ConfigError::UnableToDecode("host: port")))?;
        Ok(Self { hostname, port })
    }
}
