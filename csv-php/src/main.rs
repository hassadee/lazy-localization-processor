use std::io;
use std::process;

fn main() {
	let mut reader = csv::Reader::from_reader(io::stdin());

	for result in reader.records() {

		match result {
			Ok(record) => println!("{:?}", record),
			Err(err) => {
				println!("Error reading CSV from <stdin>: {}", err);
				process::exit(1);
			}
		}
	}
}
