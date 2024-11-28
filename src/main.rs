use swift_package::{build_cli, CliArgs};

fn main() {
    let args = CliArgs::from_env_or_exit();

    if let Err(e) = crate::build_cli(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
