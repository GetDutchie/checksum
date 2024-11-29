use clap::Parser;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    path: Option<PathBuf>,
}

fn sort_json(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            for val in arr.iter_mut() {
                sort_json(val);
            }
            arr.sort_by_key(|a| a.to_string());
        }
        Value::Object(map) => {
            let mut sorted_map = BTreeMap::new();
            for (key, val) in map.iter_mut() {
                let mut val = val.take();
                sort_json(&mut val);
                sorted_map.insert(key.clone(), val);
            }
            *map = sorted_map.into_iter().collect();
        }
        // Do nothing for primitive values
        _ => {}
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.path.as_deref() {
        let file_contents = std::fs::read_to_string(path).unwrap();
        let mut json: Value = serde_json::from_str(&file_contents).unwrap();
        sort_json(&mut json);
        let digest = md5::compute(json.to_string());
        println!("MD5 Digest: {:x}", digest);
    }
}
