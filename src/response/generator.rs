use serde_json::{json, Map, Value};
use std::collections::HashMap;

use super::faker::generate_fake_value;

pub fn generate_response_body(
    template: &Value,
    params: &HashMap<String, String>,
    request_body: Option<&Value>,
) -> Value {
    process_value(template, params, request_body)
}

fn process_value(
    value: &Value,
    params: &HashMap<String, String>,
    request_body: Option<&Value>,
) -> Value {
    match value {
        Value::Object(obj) => {
            // Check for special directives
            if let Some(fake_type) = obj.get("$fake") {
                return generate_fake_value(fake_type.as_str().unwrap_or("string"), obj);
            }

            if let Some(param_name) = obj.get("$param") {
                if let Some(name) = param_name.as_str() {
                    if let Some(param_value) = params.get(name) {
                        return Value::String(param_value.clone());
                    }
                }
                return Value::Null;
            }

            if let Some(body_field) = obj.get("$body") {
                if let Some(field_name) = body_field.as_str() {
                    if let Some(body) = request_body {
                        if let Some(field_value) = body.get(field_name) {
                            return field_value.clone();
                        }
                    }
                }
                return Value::Null;
            }

            if let Some(array_config) = obj.get("$array") {
                return generate_array(array_config, obj, params, request_body);
            }

            // Regular object - process all fields
            let mut result = Map::new();
            for (key, val) in obj {
                if key.starts_with('$') && key != "$array" {
                    continue;
                }
                result.insert(key.clone(), process_value(val, params, request_body));
            }
            Value::Object(result)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| process_value(v, params, request_body)).collect())
        }
        _ => value.clone(),
    }
}

fn generate_array(
    config: &Value,
    _obj: &Map<String, Value>,
    params: &HashMap<String, String>,
    request_body: Option<&Value>,
) -> Value {
    let config_obj = config.as_object();
    
    let count = config_obj
        .and_then(|c| c.get("count"))
        .and_then(|c| c.as_u64())
        .unwrap_or(5) as usize;

    let default_template = json!({});
    let template = config_obj
        .and_then(|c| c.get("template"))
        .unwrap_or(&default_template);

    let items: Vec<Value> = (0..count)
        .map(|_| process_value(template, params, request_body))
        .collect();

    Value::Array(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_param() {
        let template = json!({
            "id": { "$param": "userId" }
        });
        let mut params = HashMap::new();
        params.insert("userId".to_string(), "123".to_string());
        
        let result = generate_response_body(&template, &params, None);
        assert_eq!(result["id"], "123");
    }

    #[test]
    fn test_process_body_field() {
        let template = json!({
            "name": { "$body": "userName" }
        });
        let params = HashMap::new();
        let body = json!({ "userName": "John" });
        
        let result = generate_response_body(&template, &params, Some(&body));
        assert_eq!(result["name"], "John");
    }
}
