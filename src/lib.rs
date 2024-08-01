use thiserror::Error;
use toml::Value;

type TqResult<T> = std::result::Result<T, TqError>;

#[derive(Error, Debug)]
pub enum TqError {
    #[error("Failed to open file \"{file_name}\": {cause}")]
    FileOpenError { file_name: String, cause: String },

    #[error("Failed to parse TOML file \"{file_name}\": {cause}")]
    TomlParseError { file_name: String, cause: String },

    #[error("Could not find pattern {pattern}")]
    PatternNotFoundError { pattern: String },
}

pub fn extract_pattern<'a>(toml_file: &'a Value, pattern: &str) -> TqResult<&'a Value> {
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
