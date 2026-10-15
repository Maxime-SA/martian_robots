use std::env::args;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use martian_robots::*;

// talk about how I would use clap for CLI
// talk about how I would make this interative
struct ExecutableArgs {
    input_path: PathBuf,
}

fn parse_args() -> Option<ExecutableArgs> {
    let mut args = args();

    args.next(); // ignore executable name

    args.next().map(|input_path| ExecutableArgs {
        input_path: PathBuf::from(input_path.trim()),
    })
}

fn read_input_file(path: PathBuf) -> String {
    match fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => {
            eprint!("could not read input file: {}", err);
            exit(1);
        }
    }
}

fn main() -> Result<()> {
    let Some(executable_input) = parse_args() else {
        eprintln!("could not parse input file path");
        exit(1);
    };

    // talk about how I would load the file in a iterable manner
    let input = read_input_file(executable_input.input_path);

    let mut iter = input.lines();

    let (max_col, max_row) = {
        let Some(world_dimensions_line) = iter.next() else {
            eprintln!("input file missing world dimensions");
            exit(1);
        };

        parse_world_dimensions(world_dimensions_line)?
    };

    let mut mars = Mars::new(max_col, max_row);

    while let Some(robot_position) = iter.next()
        && let Some(robot_instructions) = iter.next()
    {
        let position = parse_robot_position(robot_position)?; // should we ignore this and continue
        let instructions = parse_robot_instructions(robot_instructions)?; // should we ignore this and continue

        let robot_id = mars.create_robot(position)?; // should we ignore this and continue
        let final_position = mars.execute_instructions(&robot_id, &instructions)?; // should we ignore this and continue

        println!("{final_position}");
    }

    Ok(())
}
