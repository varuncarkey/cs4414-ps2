use std::io::buffered::BufferedReader;
use std::io::stdin;

fn main(){
	// println("Printing first");
	let mut stdin = BufferedReader::new(stdin());
	// println("printing second");
	let line = match stdin.read_line(){
		Some(l)=>{l},
		None=>"".to_str()

	};
	println!("Standard Input: {}", line);
}