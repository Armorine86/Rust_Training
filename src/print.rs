pub fn run(msg: &str) {

	println!("\n==================================");
	println!("|| SOME BASIC PRINTS TO CONSOLE ||");
	println!("==================================");
	
	// simple Print to output
	println!("{}", msg);

	// Basic Formatting
	println!("{} is learning the {} language", "Maxime", "Rust");

	// Positional Arguments
	println!("{0} is learning {1} and {0} likes it very {2}",
	        "Maxime",       "Rust",                    "much");

	// Named Arguments
	println!("{name} thinks {lang1} is better than {lang2}",
				name = "Maxime",
				lang1 = "Rust",
				lang2 = "C++");

	// Placeholder Traits
	println!("Binary: {:b}\nHex: {:x}\nOcto: {:o}", 42, 42, 42);

	// Placeholder for Debug trait
	println!("{:?}", (12, true, 42, false, "DEBUG MODE"));

	// Basic Maths
	println!("21 + 21 = {}", 21 + 21);
}

