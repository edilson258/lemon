use toml::Value;

pub fn get_toml_text_value(value: &Value, key: &str) -> Result<String, String> {
	match value.get(key) {
		Some(value) => match value.as_str() {
			Some(value) => Ok(value.to_string()),
			None => Err(format!("expected string for key {}", key)),
		},
		None => Err(format!("missing key {}", key)),
	}
}

pub fn get_toml_array_value(value: &Value, key: &str) -> Result<Vec<String>, String> {
	match value.get(key) {
		Some(value) => match value.as_array() {
			Some(value) => Ok(value.iter().map(|v| v.as_str().unwrap().to_string()).collect()),
			None => Err(format!("expected array for key {}", key)),
		},
		None => Err(format!("missing key {}", key)),
	}
}

pub fn get_toml_bool_value(value: &Value, key: &str) -> Result<bool, String> {
	match value.get(key) {
		Some(value) => match value.as_bool() {
			Some(value) => Ok(value),
			None => Err(format!("expected bool for key {}", key)),
		},
		None => Err(format!("missing key {}", key)),
	}
}

pub fn get_toml_integer_value(value: &Value, key: &str) -> Result<usize, String> {
	match value.get(key) {
		Some(value) => match value.as_integer() {
			Some(value) => Ok(value as usize),
			None => Err(format!("expected integer for key {}", key)),
		},
		None => Err(format!("missing key {}", key)),
	}
}
