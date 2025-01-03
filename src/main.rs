use checksum::digest::compute_digest;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.path.as_deref() {
        let path_ext = path.extension().unwrap();
        let file_contents = std::fs::read_to_string(path).unwrap();
        let digest = compute_digest(&file_contents, path_ext.to_str());
        println!("{:?}", digest);
    }
}
