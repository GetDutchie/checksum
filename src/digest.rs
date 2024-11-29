use md5::Digest;
use serde_json::Value;
use std::collections::BTreeMap;

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

fn compute_json_digest(s: &str) -> Digest {
    let mut json: Value = serde_json::from_str(s).unwrap();
    sort_json(&mut json);
    md5::compute(json.to_string())
}

pub fn compute_digest(s: &str, ext: Option<&str>) -> Digest {
    match ext {
        Some("json") => compute_json_digest(s),
        _ => panic!("Unsupported file extension: {:?}", ext),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = compute_json_digest(
            r#"
            {"a": 1, "b": 2}
        "#,
        );
        let b = compute_json_digest(
            r#"
            {"b": 2, "a": 1}
            "#,
        );
        assert_eq!(a, b);
        let a = compute_json_digest(
            r#"
            [
                {"foo": 1, "bar": 2},
                {"baz": 3, "bop": 4},
                {"zip": 5, "zap": 6}
            ]
        "#,
        );
        let b = compute_json_digest(
            r#"
            [
                {"baz": 3, "bop": 4},
                {"zip": 5, "zap": 6},
                {"foo": 1, "bar": 2}
            ]
            "#,
        );
        assert_eq!(a, b);
    }
}
