use clap::Parser;
use std::process::exit;
use tq::OutputType;

fn main() {
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
