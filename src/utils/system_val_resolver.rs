use serde_json::Value;
use std::env::consts::OS;

pub fn resolve_system_dependent_value_config(val: &Value) -> Vec<String> {
    let mut v = Vec::new();

    match val {
        Value::String(_) | Value::Array(_) => {
            push_arr_and_str_val(val, &mut v);
        }
        Value::Object(ref values_map) => {
            for (key, val) in values_map.iter() {
                if let Some(_) = key.split("+").find(|p| p.trim() == OS) {
                    push_arr_and_str_val(val, &mut v);
                    break;
                }
            }
        }
        _ => (),
    }

    v
}

fn push_arr_and_str_val(val: &Value, v: &mut Vec<String>) {
    match val {
        Value::String(ref val) => v.push(val.clone()),
        Value::Array(ref values) => {
            for val in values.iter().filter_map(|v| v.as_str()) {
                v.push(val.to_string());
            }
        }
        _ => (),
    }
}
