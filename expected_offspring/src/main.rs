use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
	//Read File into a string-like variable
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
	
	let contents = fs::read_to_string(filename)
			.expect("File Read Error");
	println!("{}",contents);
	
	//Input contents, Conv to numbers
	let reader = io::stdin();
	let numbers: Vec<f32> =
		reader.lock()
			.lines().next().unwrap().unwrap()
			.split(' ').map(|s| s.trim())
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect();

	
	println!("{}",convert(numbers[0],numbers[1],numbers[2],numbers[3],numbers[4],numbers[5]));

	println!("Hello, world!");
}

fn convert(a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, _a6: f32) -> f32 {
	2.0*a1 + 2.0*a2 + 2.0*a3 + 1.5*a4 + a5
}