use clap::Parser;
use hypr_config_manager::templater::{generate_config, reload_applications};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    init: bool,

    #[arg(long, default_value_t = false)]
    backup: bool,

    #[arg(long, default_value_t = false)]
    no_reload: bool,

    #[arg(long)]
    targets: Option<Vec<String>>,

    theme_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path_opt = args.theme_path.map(PathBuf::from);

    match generate_config(path_opt, args.backup, args.targets, args.debug) {
        Ok(modified) => {
            if !args.no_reload {
                let _ = reload_applications(modified);
            }
        }
        Err(e) => {
            eprintln!("Error generating config: {}", e);
            std::process::exit(1);
        }
    }
}
