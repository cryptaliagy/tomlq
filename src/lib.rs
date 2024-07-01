use std::fs::File;
use std::io::prelude::*;

use clap::Parser;
use thiserror::Error;
use toml::Value;

type Result<T> = std::result::Result<T, TqError>;

#[derive(Error, Debug)]
pub enum TqError {
    #[error("Failed to open file \"{file_name}\": {cause}")]
    FileOpenError { file_name: String, cause: String },

    #[error("Failed to parse TOML file \"{file_name}\": {cause}")]
    TomlParseError { file_name: String, cause: String },

    #[error("Could not find pattern {pattern}")]
    PatternNotFoundError { pattern: String },
}

#[derive(Default, Debug, Clone, clap::ValueEnum)]
pub enum OutputType {
    #[default]
    Toml,
    #[cfg(feature = "json")]
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The TOML File to read
    #[arg(short, long, value_name = "TOML_FILE")]
    pub file: String,

    /// Field to read from the TOML file
    pub pattern: String,

    /// The output type. Default is TOML, but supports outputting in different formats.
    #[arg(short, long, value_name = "OUTPUT_TYPE", default_value = "toml")]
    pub output: OutputType,
}

pub fn extract_pattern<'a>(toml_file: &'a Value, pattern: &str) -> Result<&'a Value> {
    if pattern.is_empty() || pattern == "." {
        return Ok(toml_file);
    }

    let pattern = pattern.trim_start_matches('.');

    pattern
        .split('.')
        .fold(Some(toml_file), |acc, key| match acc {
            Some(a) => a.get(key),
            None => None,
        })
        .ok_or_else(|| TqError::PatternNotFoundError {
            pattern: pattern.to_string(),
        })
}

pub fn load_toml_from_file(file_name: &str) -> Result<toml::Value> {
    let mut file = File::open(file_name).map_err(|e| TqError::FileOpenError {
        file_name: file_name.to_string(),
        cause: e.to_string(),
    })?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    toml::from_str::<Value>(&contents).map_err(|e| TqError::TomlParseError {
        file_name: file_name.to_string(),
        cause: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pattern() {
        let toml_file = toml::from_str(
            r#"
            [package]
            test = "test"
            "#,
        )
        .unwrap();

        let x = extract_pattern(&toml_file, "package.test").unwrap();

        assert_eq!(x, &Value::String("test".to_string()));
    }

    #[test]
    fn test_fail_extract() {
        let toml_file = toml::from_str(
            r#"
            [package]
            test = "test"
            "#,
        )
        .unwrap();

        let x = extract_pattern(&toml_file, "package.test2");

        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err().to_string(),
            "Could not find pattern package.test2"
        );
    }

    #[test]
    fn test_get_prop_with_many_tables() {
        let toml_file = toml::from_str(
            r#"
            [package]
            test = "test"
            [package2]
            test2 = "test2"
            "#,
        )
        .unwrap();

        let x = extract_pattern(&toml_file, "package.test").unwrap();

        assert_eq!(x, &Value::String("test".to_string()));
    }
}
