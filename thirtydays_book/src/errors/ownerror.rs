use std::fmt::{self, write};
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::{self, Path};

// Define our custom error enum
#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    ParseError(String), 
    MissingKey(String),
    InvalidValue(String, String),
    FileNotFound(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error : {}", err),
            ConfigError::ParseError(msg) => write!(f, "Configuation parsing error: {}", msg),
            ConfigError::MissingKey(key) => write!(f, "Missing required configuration key: {}", key),
            ConfigError::InvalidValue(key, value) => { write!(f, "Invalid value for key '{}': '{}'", key, value) }
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found at: {}", path),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        if err.kind() == ErrorKind::NotFound { 
            ConfigError::FileNotFound("Unknown path".to_string())
        } else { 
            ConfigError::Io(err) 
        }
    }
}

fn load_configuration<P: AsRef<Path>>(path: P) -> Result<String, ConfigError> {
    let mut file = File::open(path)?; // If File::open returns an io::Error, it's converted and returned early. 
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.contains("invalid_syntax") { 
        return Err(ConfigError::ParseError("Encountered invalid syntax marker".to_string()));
    }

    if !contents.contains("server_port") { 
        return Err(ConfigError::MissingKey("server_port".to_string())); 
    }

    if contents.contains("timeout=abc") { 
        return Err(ConfigError::InvalidValue("timeout".to_string(), "abc".to_string())); 
    }

    Ok(contents)

}

fn main() {
    println!("--- Attempting to load non-existent config ---");
    match load_configuration("non_existent-config.toml") {
        Ok(config) => println!("Configuration loaded successfully: {}", config),
        Err(e) => match e {
            ConfigError::FileNotFound(path) => println!("Error: Configuration file not found at '{}'. Please ensure the file exists.", path), ConfigError::Io(io_err) => println!("Error: An unexpected I/O error occurred: {}", io_err), _ => println!("An unexpected error occurred: {}", e),
        }
    }

    println!("-----------------------------------------------");

}