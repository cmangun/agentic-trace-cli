use serde_json::Value;

pub fn canonicalize(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut sorted: Vec<_> = map.iter().collect();
            sorted.sort_by_key(|(k, _)| k.clone());
            let entries: Vec<String> = sorted
                .into_iter()
                .map(|(k, v)| format!("{}:{}", serde_json::to_string(k).unwrap(), canonicalize(v)))
                .collect();
            format!("{{{}}}", entries.join(","))
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(|v| canonicalize(v)).collect();
            format!("[{}]", items.join(","))
        }
        _ => serde_json::to_string(value).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_key_sorting() {
        let input = json!({"zebra": 1, "apple": 2, "mango": 3});
        assert_eq!(canonicalize(&input), r#"{"apple":2,"mango":3,"zebra":1}"#);
    }

    #[test]
    fn test_nested_sorting() {
        let input = json!({"b": {"z": 1, "a": 2}, "a": 1});
        assert_eq!(canonicalize(&input), r#"{"a":1,"b":{"a":2,"z":1}}"#);
    }
}
