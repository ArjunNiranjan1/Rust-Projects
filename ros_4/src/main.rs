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
	let numbers: Vec<i32> =
		reader.lock()
			.lines().next().unwrap().unwrap()
			.split(' ').map(|s| s.trim())
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect();
	
	//Calculations
	let k = numbers[0] as f32;
	let m = numbers[1] as f32;
	let n = numbers[2] as f32;
	let total = k + m + n;
	let out: f32 = 1.0/(total * (total - 1.0))*(k*(k-1.0) + 2.0*k*m + 2.0*k*n + m*n + 0.75*m*(m-1.0));
	
	//Output
	println!("Out: {}", out);

	
}
