use std::process;

// Abuse floating -> int cast undefined behavior to get "random" data.
// (Actually the low 8 bits from RAX on x86_64 win10 w/ debug build)
// https://github.com/rust-lang/rust/issues/10184
fn get_rand() -> u8 {
	1337.0f32 as u8
}

fn main() {
	// Return the unsound/"random" byte via the process exit code.
	process::exit(get_rand() as i32);
}
