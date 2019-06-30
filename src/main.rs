use badrng::BadRng;

fn main() {
	let rng = BadRng::new();

	let mut bytes = Vec::new();
	for _i in 0..100{
		bytes.push(rng.get_byte());
	}

	println!("{:?}", bytes);
}


