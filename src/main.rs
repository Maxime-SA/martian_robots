use std::env::args;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use martian_robots::*;

struct ExecutableArgs {
    input_path: PathBuf,
}

fn parse_args() -> Option<ExecutableArgs> {
    args().nth(1).map(|input_path| ExecutableArgs {
        input_path: PathBuf::from(input_path.trim()),
    })
}

fn read_input_file(path: PathBuf) -> String {
    match fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("could not read input file: {}", err);
            exit(1);
        }
    }
}

fn main() -> Result<()> {
    let Some(executable_input) = parse_args() else {
        eprintln!("could not parse arguments");
        exit(1);
    };

    let mut stdout = std::io::stdout().lock();

    Mars::parse_and_execute(
        read_input_file(executable_input.input_path).as_str(),
        &mut stdout,
    )
}
