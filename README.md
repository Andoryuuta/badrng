# badrng
An intentionally bad "random"(not really) number generator in Rust.

## DONT USE THIS!
If you need random numbers, use the [`rand` crate](https://crates.io/crates/rand)!

I was just having some fun abusing the six-year-old issue @ https://github.com/rust-lang/rust/issues/10184 to generate "random" data.
The data isn't actually random, but instead just the lower 8 bits of the RAX register on x86_64 (when built in debug mode only). 
