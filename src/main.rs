use anyhow::Result;
use checksum::digest::compute_digest;
use clap::Parser;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to file to compute digest for
    #[arg(short, long, value_name = "FILE")]
    path: Option<PathBuf>,
    /// Input string to compute digest for
    #[arg(short, long, value_name = "INPUT")]
    input: Option<String>,
    /// File type of supplied input. Supported file extensions: json, yaml, yml
    #[arg(short, long, value_name = "FILETYPE")]
    r#type: Option<String>,
}

fn handle_path(path: &Path) -> Result<(), anyhow::Error> {
    if let Some(path_ext) = path.extension() {
        let file_contents = std::fs::read_to_string(path)?;
        let digest = compute_digest(&file_contents, path_ext.to_str())?;
        println!("{:?}", digest);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Unable to extract file extension for {:?}",
            path
        ))?
    }
}

fn handle_input(input: &str, filetype: Option<&str>) -> Result<(), anyhow::Error> {
    let digest = compute_digest(input, filetype)?;
    println!("{:?}", digest);
    Ok(())
}

fn handle_terminal(filetype: Option<&str>) -> Result<(), anyhow::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let digest = compute_digest(buffer.trim(), filetype)?;
    println!("{:?}", digest);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(path) = cli.path.as_deref() {
        handle_path(path)
    } else if let Some(input) = cli.input.as_deref() {
        handle_input(input, cli.r#type.as_deref())
    } else if !io::stdin().is_terminal() {
        handle_terminal(cli.r#type.as_deref())
    } else {
        Err(anyhow::anyhow!(
            "Unable to output digest with supplied command line args"
        ))?
    }
}
