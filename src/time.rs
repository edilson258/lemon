pub fn format_time(time: std::time::Duration, force_seconds: bool) -> String {
	if force_seconds {
		return format!("{:.4}s", time.as_secs_f32());
	}

	if time.as_secs_f64() >= 1.0 {
		return format!("{:.3}s", time.as_secs_f64());
	}

	if time.subsec_millis() > 0 {
		return format!("{}ms", time.subsec_millis());
	}

	if time.subsec_micros() > 0 {
		return format!("{}us", time.as_micros());
	}

	format!("{}ns", time.as_nanos())
}
