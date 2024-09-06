//! Legacy tomlq binary.

use clap::Parser;
#[cfg(feature = "color")]
use colored::Colorize;
use std::process::exit;


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

fn main() {
    let warning = "You are currently using the legacy version of tomlq. Please use the new binary \"tq\" instead.";
    let deprecation = "The \"tomlq\" binary will be removed from this package starting in version 0.2.0, scheduled for January 1, 2025";

    #[cfg(feature = "color")]
    let warning = warning.yellow();

    #[cfg(feature = "color")]
    let deprecation = deprecation.yellow();

    eprintln!("{}", warning,);
    eprintln!("{}", deprecation);

    let app = Cli::parse();

    let toml_file = tq::load_toml_from_file(&app.file);

    let Ok(toml_file) = toml_file else {
        let e = toml_file.unwrap_err();
        eprintln!("{}", e);
        exit(-1);
    };

    let x = tq::extract_pattern(&toml_file, &app.pattern);

    exit(match x {
        Ok(needle) => {
            match app.output {
                OutputType::Toml => println!("{}", format!("{}", needle).trim_matches('"')),
                #[cfg(feature = "json")]
                OutputType::Json => println!("{}", serde_json::to_string(&needle).unwrap()),
            }
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    });
}
