mod pickers;

use std::process::exit;

use pickers::get_command;

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
/// Get command to run to run specific test in a file
struct Opts {
    // /// specify runner override default runner
    // #[argh(option, long = "runner")]
    // runner: Option<String>,
    /// run full test suite
    #[argh(switch, long = "full")]
    full: bool,

    /// name of the test file
    #[argh(positional)]
    filename: String,

    /// line number in the test file
    #[argh(positional)]
    line_no: Option<usize>,
}

fn main() -> Result<()> {
    let args: Opts = argh::from_env();

    let te = get_command(&args.filename, args.line_no, args.full)?;
    if let Some(t) = te {
        println!("{}", t);
    } else {
        eprintln!("Unable to find any tests");
        exit(1);
    }
    Ok(())
}
