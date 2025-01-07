use anyhow::Result;
use md5::Digest;
use serde_json::Value;
use serde_yaml;
use std::collections::BTreeMap;

fn sort(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            for val in arr.iter_mut() {
                sort(val);
            }
            arr.sort_by_key(|a| a.to_string());
        }
        Value::Object(map) => {
            let mut sorted_map = BTreeMap::new();
            for (key, val) in map.iter_mut() {
                let mut val = val.take();
                sort(&mut val);
                sorted_map.insert(key.clone(), val);
            }
            *map = sorted_map.into_iter().collect();
        }
        // Do nothing for primitive values
        _ => {}
    }
}

fn compute_json_digest(s: &str) -> Result<Digest> {
    let json: Result<Value, serde_json::Error> = serde_json::from_str(s);
    match json {
        Ok(mut data) => {
            sort(&mut data);
            Ok(md5::compute(data.to_string()))
        }
        Err(_) => Err(anyhow::anyhow!("Unable to parse json contents"))?,
    }
}

fn compute_yaml_digest(s: &str) -> Result<Digest> {
    let yaml: Result<Value, serde_yaml::Error> = serde_yaml::from_str(s);
    match yaml {
        Ok(mut data) => {
            sort(&mut data);
            Ok(md5::compute(data.to_string()))
        }
        Err(_) => Err(anyhow::anyhow!("Unable to parse yaml contents"))?,
    }
}

/// # Example
/// ```
/// use checksum::digest::compute_digest;
/// use md5::Digest;
/// // regardless of the order of the keys, the digest should be the same
/// let digest_a: Digest = compute_digest("{\"a\": 1, \"b\": 2}", Some("json")).unwrap();
/// let digest_b: Digest = compute_digest("{\"b\": 2, \"a\": 1}", Some("json")).unwrap();
/// assert_eq!(digest_a, digest_b);
/// ```
pub fn compute_digest(s: &str, ext: Option<&str>) -> Result<Digest> {
    match ext {
        Some("json") => compute_json_digest(s),
        Some("yaml") | Some("yml") => compute_yaml_digest(s),
        _ => {
            if let Some(ext) = ext {
                panic!("Unsupported file type, {:?}", ext);
            } else {
                panic!("No file extension provided");
            }
        }
    }
}
