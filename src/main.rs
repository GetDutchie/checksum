use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.path.as_deref() {
        let file_contents = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&file_contents).unwrap();
        let digest = md5::compute(json.to_string());
        println!("{:?}", digest);
    }
}
