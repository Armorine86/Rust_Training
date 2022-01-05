// ** COMMAND LINE ARGUMENTS **

pub fn run() {

	// Here, args() is called an iterator. It will iterates trought all program arguments one by one
	// collect() transforms the iterator into a Vector containing all the elements
	let arguments: Vec<String> = std::env::args().skip(1).collect();

	let mut file = std::fs::File::create("Test.txt");

	write!(file, "{}", arguments.join(", ")); // TODO Make it work
	println!("Program Arguments: {:#?}", arguments)
	
}
