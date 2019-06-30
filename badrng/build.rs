// build.rs

use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=bad-rng-internal-bin/src/main.rs");
	
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("cargo install --debug --force --path ./bad-rng-internal-bin")
            .output()
            .expect("Failed to install bad-rng-internal-bin");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cargo install --debug --force --path ./bad-rng-internal-bin")
            .output()
            .expect("Failed to install bad-rng-internal-bin");
    };
}
