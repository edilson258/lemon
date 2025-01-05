pub enum Machine {
	Win,
	Linux,
	Mac,
	Unknown,
}

#[inline(always)]
pub fn get_current_user_machine() -> Machine {
	if cfg!(target_os = "windows") {
		Machine::Win
	} else if cfg!(target_os = "linux") {
		Machine::Linux
	} else if cfg!(target_os = "macos") {
		Machine::Mac
	} else {
		Machine::Unknown
	}
}

#[inline(always)]
#[allow(dead_code)]
pub fn is_std_lib(name: &str) -> bool {
	matches!(name, "print" | "println" | "panic" | "exit")
}
