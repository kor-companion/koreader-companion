mod cli;
mod output;
mod report;

use std::env;
use std::path::Path;

use cli::{parse_command, Command};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let result = match parse_command(&args) {
        Ok(Command::Foundation) => report::foundation_report(),
        Ok(Command::Probe { path }) => report::probe_path(Path::new(&path)),
        Err(error) => Err(error),
    };

    if let Err(error) = result {
        output::print_usage();
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
