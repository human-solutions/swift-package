use clap::Parser;
use std::env;
use swift_package::{build, SpmCli};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // when running as cargo leptos, the second argument is "leptos" which
    // clap doesn't expect
    if args.get(1).map(|a| a == "swift-package").unwrap_or(false) {
        args.remove(1);
    }

    let args = SpmCli::parse_from(&args);
    if let Err(e) = crate::build(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
