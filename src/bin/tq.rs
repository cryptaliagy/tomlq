use clap::Parser;
use serde::ser::Serialize;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use toml::{ser::ValueSerializer, Value};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The TOML File to read. Omit this flag to read from STDIN
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// The output type. Default is TOML, but supports outputting in different formats.
    #[arg(short, long, default_value = "toml")]
    pub output: Format,

    /// Should the output be raw? Only applies to TOML output.
    #[arg(short, long)]
    pub raw: bool,

    /// The input type. Default is TOML, but supports inputting in different formats. If the input
    /// is JSON, it will be converted to TOML. So there is an overhead of using JSON input.
    #[arg(short, long, default_value = "toml")]
    pub input: Format,

    /// Should "pretty" printing be used? Only applies to JSON output.
    #[cfg(feature = "json")]
    #[arg(short, long)]
    pub pretty: bool,

    /// Field to read from the TOML file
    pub pattern: String,

    #[cfg(feature = "syntax-highlighting")]
    #[arg(short, long, default_value = "auto")]
    pub color: clap::ColorChoice,
}

#[derive(Default, Debug, Copy, Clone, clap::ValueEnum)]
enum Format {
    #[default]
    Toml,

    #[cfg(feature = "json")]
    Json,
}

fn main() -> anyhow::Result<()> {
    let app = Cli::parse();

    #[cfg(feature = "syntax-highlighting")]
    match app.color {
        // console will by default respect certain environment variables for color output,
        // in addition to checking if the standard output is a TTY.
        clap::ColorChoice::Auto => {}
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

    let input_string = match app.input {
        Format::Toml => input_string,
        #[cfg(feature = "json")]
        Format::Json => {
            if let Ok(json_value) = serde_json::from_str::<toml::Value>(&input_string) {
                // If the input is JSON, convert it to TOML
                toml::to_string(&json_value)?
            } else {
                input_string
            }
        }
    };
    let toml_value: toml::Value = toml::from_str(&input_string)?;

    let result: &Value = tq::extract_pattern(&toml_value, &app.pattern)?;

    #[cfg(feature = "json")]
    let pretty = app.pretty;

    #[cfg(not(feature = "json"))]
    let pretty = false;

    // Generate a string to print
    let output = match (app.output, pretty) {
        (Format::Toml, _) => {
            let mut output = String::new();
            let serializer = ValueSerializer::new(&mut output);
            result.serialize(serializer)?;
            output
        }

        #[cfg(feature = "json")]
        (Format::Json, false) => serde_json::to_string(result)?,

        #[cfg(feature = "json")]
        (Format::Json, true) => serde_json::to_string_pretty(result)?,
    };

    #[cfg(feature = "syntax-highlighting")]
    if !app.raw {
        // If the syntax-highlighting crate feature is enabled, use `bat`'s pretty printing system to print with
        // highlighting. This will not restructure code/lines, and does not override the --pretty flag.
        let mut pretty_printer = bat::PrettyPrinter::new();

        pretty_printer
            .colored_output(console::colors_enabled())
            .grid(false)
            .rule(false)
            .line_numbers(false);

        match app.output {
            Format::Toml => {
                pretty_printer
                    .language("toml")
                    .input_from_bytes(output.as_bytes())
                    .print()?;
            }

            #[cfg(feature = "json")]
            Format::Json => {
                pretty_printer
                    .language("json")
                    .input_from_bytes(output.as_bytes())
                    .print()?;
            }
        }
    } else {
        println!("{}", output.trim_matches('"'));
    }

    // If there is not syntax highlighting, just print normally.
    #[cfg(not(feature = "syntax-highlighting"))]
    if !app.raw {
        println!("{output}");
    } else {
        println!("{}", output.trim_matches('"'));
    }

    Ok(())
}
