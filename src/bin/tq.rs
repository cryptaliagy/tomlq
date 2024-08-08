use clap::Parser;
use std::process::ExitCode;
use tq::OutputType;

fn main() -> ExitCode {
    let app = tq::Cli::parse();

    let toml_file = tq::load_toml_from_file(&app.file);

    let Ok(toml_file) = toml_file else {
        let e = toml_file.unwrap_err();
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    };

    let x = tq::extract_pattern(&toml_file, &app.pattern);

    match x {
        Ok(needle) => {
            match app.output {
                OutputType::Toml => println!("{}", format!("{}", needle).trim_matches('"')),
                #[cfg(feature = "json")]
                OutputType::Json => println!("{}", serde_json::to_string(&needle).unwrap()),
            }

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
