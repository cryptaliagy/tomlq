use clap::Parser;
use toml::Value;
use std::{fs::File, io::{self, Read}, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The TOML File to read. Omit this flag to read from STDIN
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// The output type. Default is TOML, but supports outputting in different formats.
    #[arg(short, long, default_value = "toml")]
    pub output: OutputType,

    /// Should "pretty" printing be used?
    #[arg(short, long)]
    pub pretty: bool,

    /// Field to read from the TOML file
    pub pattern: String,

    #[cfg(feature = "syntax-highlighting")]
    #[arg(short, long, default_value = "auto")]
    pub color: clap::ColorChoice
}

#[derive(Default, Debug, Copy, Clone, clap::ValueEnum)]
enum OutputType {
    #[default]
    Toml,

    /// Print JSON all on one line.
    #[cfg(feature = "json")]
    Json,
}

fn main() -> anyhow::Result<()> {
    let app = Cli::parse();

    #[cfg(feature = "syntax-highlighting")]
    match app.color {
        // console will by default respect certain environment variables for color output, 
        // in addition to checking if the standard output is a TTY.
        clap::ColorChoice::Auto => {},
        clap::ColorChoice::Never => console::set_colors_enabled(false),
        clap::ColorChoice::Always => console::set_colors_enabled(true),
    }

    // Get a reader over the input to tq.
    let mut reader: Box<dyn Read> = match &app.file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };

    let mut input_string = String::new();
    reader.read_to_string(&mut input_string)?;

    let toml_value: toml::Value = toml::from_str(&input_string)?;

    let result: &Value = tq::extract_pattern(&toml_value, &app.pattern)?;

    // Generate a string to print
    let output = match (app.output, app.pretty) {
        (OutputType::Toml, false) => toml::to_string(result)?,
        (OutputType::Toml, true) => toml::to_string_pretty(result)?,

        #[cfg(feature = "json")]
        (OutputType::Json, false) => serde_json::to_string(result)?,

        #[cfg(feature = "json")]
        (OutputType::Json, true) => serde_json::to_string_pretty(result)?,
    };

    #[cfg(feature = "syntax-highlighting")] {
        // If the syntax-highlighting crate feature is enabled, use `bat`'s pretty printing system to print.
        let mut pretty_printer = bat::PrettyPrinter::new();

        pretty_printer
            .colored_output(console::colors_enabled())
            .grid(false)
            .rule(false)
            .line_numbers(false);

        match app.output {
            OutputType::Toml => {
                pretty_printer
                    .language("toml")
                    .input_from_bytes(output.as_bytes())
                    .print()?;
            }

            #[cfg(feature = "json")]
            OutputType::Json => {
                pretty_printer
                    .language("json")
                    .input_from_bytes(output.as_bytes())
                    .print()?;
            }
        }
    }

    // If there is not syntax highlighting, just print normally.
    #[cfg(not(feature = "syntax-highlighting"))]
    println!("{output}");

    Ok(())
}
