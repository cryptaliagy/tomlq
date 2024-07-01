use clap::Parser;
use colored::Colorize;
use std::process::exit;
use tq::OutputType;

fn main() {
    eprintln!(
        "{}",
        "You are currently using the legacy version of tomlq. Please use the new binary \"tq\" instead.".yellow()
    );
    eprintln!(
        "{}",
        "The \"tomlq\" binary will be removed from this package starting in version 0.2.0, scheduled for January 1, 2025".yellow()
    );

    let app = tq::Cli::parse();

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
