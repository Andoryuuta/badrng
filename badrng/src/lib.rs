use std::process::Command;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread;

fn get_unsound_byte() -> u8 {
    // Call into the bad-rng-internal-bin binary to get a "random" byte.
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("bad-rng-internal-bin")
            .status()
            .expect("Failed to run bad-rng-internal-bin")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("bad-rng-internal-bin")
            .status()
            .expect("Failed to run bad-rng-internal-bin")
    };

    // Get the data from the error code.
    status.code().unwrap() as u8
}

fn threaded_get_unsound_bytes(n: usize) -> Vec<u8> {
	let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let mut workers = Vec::new();

    // Spawn a worker thread for each unsound "random" byte.
    for _ in 0..n {
        let thread_tx = tx.clone();
        let worker = thread::spawn(move || {
            thread_tx.send(get_unsound_byte()).unwrap();
        });

        workers.push(worker);
    }

    let mut ubs = Vec::with_capacity(n);
    for _ in 0..n {
    	ubs.push(rx.recv().unwrap());
    }

    ubs
}

pub struct BadRng {}
impl BadRng {
    pub fn new() -> Self {
        Self{}
    }

    pub fn get_byte(&self) -> u8 {
        // Get 8 unsound bytes, one for each bit we need to randomize.
        let ubs = threaded_get_unsound_bytes(8);

		// In the tested environment, the unsound bytes seem equally distributed,
        // but are 4 byte aligned, so we instead use the value to determine if individual bits
        // should be set, resulting in a better distribution of numbers.
        let mut out: u8 = 0;
        for (i, ub) in ubs.iter().enumerate() {
			if *ub < 128 {
                out |= 1 << i
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::BadRng;

    #[test]
    fn main() {
        let rng = BadRng::new();
        for _i in 1..10 {
            println!("{:?}", rng.get_byte());
        }
    }
}
