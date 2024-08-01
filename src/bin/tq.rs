use clap::{ColorChoice, Parser};
use toml::Value;
use std::{fs::File, io::{self, Cursor, Read}, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The TOML File to read, or STDIN if none is supplied.
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// The output type. Default is TOML, but supports outputting in different formats.
    #[arg(short, long, default_value = "toml")]
    pub output: OutputType,

    /// Field to read from the TOML file
    pub pattern: String,

    #[arg(short, long, default_value = "auto")]
    pub color: ColorChoice
}

#[derive(Default, Debug, Clone, clap::ValueEnum)]
pub enum OutputType {
    #[default]
    Toml,
    #[cfg(feature = "json")]
    Json,
}

fn main() -> anyhow::Result<()> {
    let app = Cli::parse();

    match app.color {
        // console will by default respect certain environment variables for color output, 
        // in addition to checking if the standard output is a TTY.
        ColorChoice::Auto => {},
        ColorChoice::Never => console::set_colors_enabled(false),
        ColorChoice::Always => console::set_colors_enabled(true),
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

    // Use `bat`'s pretty printing system to print.
    let mut pretty_printer = bat::PrettyPrinter::new();
    pretty_printer
        .colored_output(console::colors_enabled())
        .grid(false)
        .rule(false)
        .line_numbers(false);

    match app.output {
        OutputType::Toml => {
            let toml_str = toml::to_string_pretty(result)?;
            
            pretty_printer
                .language("toml")
                .input_from_reader(Cursor::new(toml_str))
                .print()?;
        },


        #[cfg(feature = "json")]
        OutputType::Json => {
            pretty_printer
                .language("json")
                .input_from_reader(Cursor::new(serde_json::to_string_pretty(result)?))
                .print()?;
        }
    }

    Ok(())
}
