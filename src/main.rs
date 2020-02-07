extern crate clap;
extern crate itch;

use clap::Clap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

// the clap docs haven't caught up to structopt yet, so read them instead
/// InTerCHanges one data format into another
#[derive(Clap)]
struct CliArgs {
    /// Format of the input, will be derived if possible
    #[clap(short = 'f', long = "from")]
    from_type: Option<itch::FromType>,

    /// Format of the output, will be derived if possible
    #[clap(short = 't', long = "to")]
    to_type: Option<itch::ToType>,

    /// Path to the input file, leave empty for stdin
    #[clap(short = 'i', long = "input", parse(from_os_str))]
    input: Option<PathBuf>,

    /// Path to the output file, leave empty for stdout
    #[clap(short = 'o', long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn open_or_create_file(path: &PathBuf) -> Result<File, std::io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                File::create(path).map_err(|e| {
                    dbg!(&e);
                    e
                })
            } else {
                Err(e)
            }
        }
    }
}

fn get_from_type_from_cli_args(cli_args: &CliArgs) -> Result<&itch::FromType, String> {
    cli_args
        .from_type
        .as_ref()
        .ok_or_else(|| "must define a from type if an input file is not specified".to_owned())
}

fn get_from_type_from_input(input: &PathBuf) -> Result<itch::FromType, String> {
    itch::FromType::from_str(
        input
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| "could not detect extension of input file".to_owned())?,
    )
    .map_err(|e| format!("error finding input format from filename ({})", e))
}

fn get_to_type_to_cli_args(cli_args: &CliArgs) -> Result<&itch::ToType, String> {
    cli_args
        .to_type
        .as_ref()
        .ok_or_else(|| "must define a to type if an input file is not specified".to_owned())
}

fn get_to_type_from_output(output: &PathBuf) -> Result<itch::ToType, String> {
    itch::ToType::from_str(
        output
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| "could not detect extension of output file".to_owned())?,
    )
    .map_err(|e| format!("error finding output format from filename ({})", e))
}

fn run() -> Result<(), String> {
    let cli_args: CliArgs = CliArgs::parse();

    match (cli_args.input.as_ref(), cli_args.output.as_ref()) {
        (Some(input), Some(output)) => itch::convert(
            &get_from_type_from_input(&input)?,
            &get_to_type_from_output(&output)?,
            File::open(input)
                .map_err(|e| format!("could not open input file to be read ({})", e))?,
            open_or_create_file(output)
                .map_err(|e| format!("could not open output file to be read ({})", e))?,
        ),

        (Some(input), None) => itch::convert(
            &get_from_type_from_input(&input)?,
            get_to_type_to_cli_args(&cli_args)?,
            File::open(input)
                .map_err(|e| format!("could not open input file to be read ({})", e))?,
            std::io::stdout(),
        ),

        (None, Some(output)) => itch::convert(
            get_from_type_from_cli_args(&cli_args)?,
            &get_to_type_from_output(&output)?,
            std::io::stdin(),
            open_or_create_file(output)
                .map_err(|e| format!("could not open output file to be read ({})", e))?,
        ),

        (None, None) => itch::convert(
            get_from_type_from_cli_args(&cli_args)?,
            get_to_type_to_cli_args(&cli_args)?,
            std::io::stdin(),
            std::io::stdout(),
        ),
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}\nrun `itch --help` for cli arguments", e);
    }
}
