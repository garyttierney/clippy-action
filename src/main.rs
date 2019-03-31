use clap::{App, Arg};
use env_logger;

mod action;
mod clippy;
mod github;

fn main() {
    env_logger::init();

    let matches = App::new("clippy-action")
        .arg(
            Arg::with_name("check-run-name")
                .short("n")
                .long("check-run-name")
                .value_name("RUN_NAME")
                .help("Override the name used in the GitHub Check (default: Clippy)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ignore-parse-errors")
                .long("ignore-parse-errors")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .get_matches();

    let run_name = matches.value_of("check-run-name").unwrap_or("Clippy");
    let clippy_output = matches.value_of("INPUT").unwrap_or("clippy.out.json");
    let ignore_parse_errors = matches.is_present("ignore-parse-errors");

    let options = action::ActionOptions::new(
        run_name.to_owned(),
        "Lints".to_owned(),
        clippy_output.to_owned(),
        ignore_parse_errors,
    );

    match action::run(options) {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}\n {}", e.as_fail(), e.backtrace());
        }
    }
}
