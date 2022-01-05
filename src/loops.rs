// ** LOOPS **

// Loops - Used to iterate until a condition is met

pub fn run() {

	println!("\n===================");
	println!("|| INFINITE LOOP ||");
	println!("===================");

	let mut i = 0;
	
	// This will run forever. Hit CTRL+C to stop the program
	loop {
		println!("i is now: {}", i);
		i += 1;
	
		// use a condition to break the loop
		if i == 10 {
			break; // gets out of the loop
		}
	}

	println!("\n================");
	println!("|| WHILE LOOP ||");
	println!("================");

	i = 0;
	while i <= 100 {
		if i % 15 == 0 {
			println!("FizzBuzz");
		} else if i % 3 == 0{
			println!("Fizz");
		} else if i % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", i);
		}
		i += 1; // While Loops need an incrementor for them to work
	}
	
	println!("\n==============");
	println!("|| FOR LOOP ||");
	println!("==============");

	// For Loops delare and initiate their own incrementor (i), in this case, initialise is to 0 and break when it reaches 100 (0..100)
	for i in 0..100 {
		if i % 15 == 0 {
			println!("FizzBuzz");
		} else if i % 3 == 0{
			println!("Fizz");
		} else if i % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", i);
		}
	}
}
