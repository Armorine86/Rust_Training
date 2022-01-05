// ** THERE IS TWO TYPES OF STRINGS **

// 1. Primitive &str = immutable fixed-length string allocated on the STACK

// 2. std::string = Growable, modifiable, heap-allocated data structure

pub fn run() {

	println!("\n===============================");
	println!("|| &STR versus STRINGS TESTS ||");
	println!("===============================");

	let hello = "Hello!"; // fixed type of &str on the STACK because we know its length before hand

	println!("Simple &str: {}\n", hello);

	// The 'From' trait allows for a type to define how to create itself from another type... a '&str' in this case
	let mut my_string = String::from("This is a String data structure type");

	println!("{}\nit's length is: {}", my_string, my_string.len());

	println!("\n======================");
	println!("|| STRING PUSH TEST ||");
	println!("======================");

	// using 'push' and 'push_str' we can append a single char or a string to the already existing string	
	my_string.push_str(" and we can add a char or even a whole string to it ! ");
	my_string.push('\u{1F604}');

	println!("{}", my_string);

	let str1 = String::from("Hello, World!");

	println!("\n=========================");
	println!("|| STRING CAPACITY TEST ||");
	println!("=========================");

	// Find a string capacity in bytes on the heap
	println!("str1 capacity in bytes is: {} bytes\n", str1.capacity());

	println!("\n===========================");
	println!("|| STRING IS EMPTY TEST ||");
	println!("===========================");

	// Check if the string is empty
	println!("is my_string emtpy? : {}\n", my_string.is_empty());


	println!("\n=====================================");
	println!("|| STRING CONTAINES SUBSTRING TEST ||");
	println!("=====================================");

	// Check is string contains substrings
	println!("is the word 'structure' in my_string? : {}", my_string.contains("structure"));
	println!("is the word 'bobby' in my_string? : {}\n", my_string.contains("bobby"));

	println!("\n=======================");
	println!("|| WORD REPLACE TEST ||");
	println!("=======================");

	// Replace a word within the string
	println!("Replace 'World' with 'There' in str1");
	println!("BEFORE: {}", str1);
	println!("AFTER: {}", str1.replace("World", "There"));

	println!("\n=======================");
	println!("|| STRING SPLIT TEST ||");
	println!("=======================");
	
	println!("BEFORE: {}", str1);
	println!("\n--AFTER--");

	// Split the string every whitespace with a newline
	for spaces in str1.split_ascii_whitespace(){
		println!("{}", spaces)
	}

	println!("\n===============================");
	println!("|| STRING WITH CAPACITY TEST ||");
	println!("===============================");

	// Create a string with a predefined capacity
	let mut str2 = String::with_capacity(10);

	str2.push('a');
	str2.push('b');
	str2.push('c');

	println!("{}", str2);

	println!("Capacity: {}", str2.capacity());

	println!("\n====================");
	println!("|| ASSERTION TEST ||");
	println!("====================");

	// assert equal lhs == rhs (left hand side == right hand side)
	assert_eq!(5, str2.len())

	// STILL A LOT MORE METHODS CONCERNING STRINGS
}
