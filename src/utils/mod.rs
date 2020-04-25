use serde_json::Value;

pub mod system_val_resolver;
pub mod var_subs;

pub fn or<T: Sized>(cond: bool, truth_val: T, false_val: T) -> T {
    if cond {
        truth_val
    } else {
        false_val
    }
}

pub fn json_val_to_actual_str(val: &Value) -> String {
    match val {
        Value::String(ref s) => s.to_string(),
        Value::Bool(i) => i.to_string(),
        Value::Number(ref n) => n.to_string(),
        Value::Null => "".to_owned(),
        Value::Array(ref a) => {
            let mut buf = String::new();
            buf.push('[');
            for i in a.iter() {
                buf.push_str(json_val_to_actual_str(i).as_str());
                buf.push_str(", ");
            }
            buf.push(']');
            buf
        }
        Value::Object(_) => "[object]".to_owned(),
    }
}
