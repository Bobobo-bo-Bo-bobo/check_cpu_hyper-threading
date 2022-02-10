#[macro_use]
extern crate simple_error;

mod constants;
mod cpu;
mod result;
mod usage;

use getopts::Options;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut warn_mode = false;

    options.optopt("H", "hyper-threading", "Check hyper-threading mode", "mode");
    options.optflag("h", "help", "Show help text");
    options.optflag("V", "version", "Show version information");
    options.optflag(
        "W",
        "warn",
        "Report warning instead of critical if hyper-threading mode does not match",
    );

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments ({})", e);
            println!();
            usage::show_usage();
            process::exit(constants::NAGIOS_UNKNOWN);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(constants::NAGIOS_OK);
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(constants::NAGIOS_OK);
    }

    let ht_mode = match opts.opt_str("H") {
        Some(v) => v.to_lowercase(),
        None => {
            eprintln!("Error: Missing hyper-threading mode to check");
            println!();
            usage::show_usage();
            process::exit(constants::NAGIOS_UNKNOWN);
        }
    };

    if ht_mode != "on" && ht_mode != "off" {
        eprintln!("Error: Invalid hyper-threading mode");
        println!();
        usage::show_usage();
        process::exit(constants::NAGIOS_UNKNOWN);
    }

    if opts.opt_present("W") {
        warn_mode = true;
    }

    let ht_enabled = match cpu::is_hyper_threading_enabled() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("UNKNOWN - {}", e);
            process::exit(constants::NAGIOS_UNKNOWN);
        }
    };

    let status = result::process_result(&ht_mode, ht_enabled, warn_mode);

    println!("{}", status.message);
    process::exit(status.status);
}
