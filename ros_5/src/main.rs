use std::env;
use std::fs;

fn main() {
	//Read File into a string-like variable
		let args: Vec<String> = env::args().collect();

		let filename = &args[1];
	
		let s = fs::read_to_string(filename)
			.expect("File Read Error");

	//Separate interesting elements
	let a: Vec<&str> = s.split("\r\n").collect();
	let mut m = String::new();
	m.push_str(&a[1]);
	let l = a[1].len();
	println!("motif: {}", m);
	println!("Length: {}", l);
	
	//Scanning Window of length l
	for t in 0..a[0].len()-l+1 {
		//println!("{}", &a[0][t..t+l]);
		match &a[0][t..t+l] {
			"ATAT" => println!("{}",t+1),
			_ => (),
		}
	}
	
}
