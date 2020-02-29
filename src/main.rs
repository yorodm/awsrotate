use clap::{crate_authors, App, Arg, ArgMatches, SubCommand};
use ctrlc;
mod util;

fn run_cli<'a>() -> ArgMatches<'a> {
    let common_args = vec![
        Arg::with_name("group")
            .short("g")
            .long("group")
            .help("Operate on streams from the specific group")
            .value_name("GROUP")
            .required(false),
        Arg::with_name("from")
            .short("f")
            .long("from")
            .value_name("FROM")
            .help("Filter streams starting on this date")
            .requires("group"),
        Arg::with_name("t")
            .short("t")
            .long("to")
            .value_name("TO")
            .requires("from")
            .help("Filter streams ending on this date"),
    ];
    App::new("awsrotate")
        .version("0.1.0")
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("list").args(&common_args))
        .subcommand(SubCommand::with_name("purge").args(&common_args))
        .get_matches()
}
fn main() {
    ctrlc::set_handler(move || std::process::exit(0))
        .expect("Could not set Ctrl+C handler...bailing out");
    run_cli();
}
