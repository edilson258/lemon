use toml::Value;

pub fn get_toml_value<T, F>(value: &Value, key: &str, convert: F) -> Result<T, String>
where
	F: Fn(&Value) -> Option<T>,
{
	value.get(key).and_then(&convert).ok_or_else(|| format!("missing or invalid key {}", key))
}

pub fn get_toml_text_value(value: &Value, key: &str) -> Result<String, String> {
	get_toml_value(value, key, |v| v.as_str().map(|s| s.to_string()))
}

pub fn get_toml_array_value(value: &Value, key: &str) -> Result<Vec<String>, String> {
	get_toml_value(value, key, |v| {
		v.as_array()
			.map(|arr| arr.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
	})
}

pub fn get_toml_bool_value(value: &Value, key: &str) -> Result<bool, String> {
	get_toml_value(value, key, Value::as_bool)
}

pub fn get_toml_integer_value(value: &Value, key: &str) -> Result<usize, String> {
	get_toml_value(value, key, |v| v.as_integer().map(|i| i as usize))
}
