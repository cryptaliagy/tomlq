use clap::Parser;
use std::process::exit;

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
            println!("{}", format!("{}", needle).trim_matches('"'));
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    });
}
