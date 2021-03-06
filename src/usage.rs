use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!(
        "Usage: {} [-V|--version] [-W|--warn] -H on|off|--hyper-threading=on|off [-h|--help]

    -H on|off                   Check if hyper-threading is on or off
    --hyper-threading=on|off

    -V                          Show version information
    --version

    -W                          Report warning condition instead of critical
    --warn                      if hyper-threading mode does not match

    -h                          Show this help text
    --help

",
        constants::NAME
    );
}
