// ** VECTORS **

// Vectors are resizable arrays

pub fn run() {

	println!("\n================================");
	println!("|| VECTOR INITILIZATION TESTS ||");
	println!("================================");

	// Vec<i32> is the structure with a specified data type (i32)
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

	println!("{:?}", numbers);

	// If you remove an element from the array, you will have a compilation error.
	// The length of the array is 'FIXED' and cannot be modified.
	// You can however modify the value inside the array as long as they remain of the same type

	println!("\n============================");
	println!("|| VECTOR RE-ASSIGN TESTS ||");
	println!("============================");

	// re-assigning array elements
	numbers[0] = 9;
	numbers[1] = 8;
	numbers[2] = 7;
	numbers[3] = 6;
	numbers[4] = 5;
	println!("{:?}", numbers);

	// numbers = [1, 2, 3]   <--  NOT GOOD
	// numbers = [1, 2, 3, 4, "A String"]   <--  NOT GOOD
	
	println!("\n=============================================");
	println!("|| VECTOR ADDING / REMOVING ELEMENTS TESTS ||");
	println!("=============================================");

	// Adding elements to a Vector
	numbers.push(10); // adds the element at the end of the Vector
	numbers.push(42);

	println!("Addind elements: {:?}", numbers);

	// Removing element from the Vector
	numbers.pop(); // Removes the last element of the vector

	println!("Removing elements: {:?}", numbers);


	println!("\n==============================");
	println!("|| ACCESS ARRAY INDEX TESTS ||");
	println!("==============================");

	// Access a single value in the array
	println!("Value at index [0] is: {}", numbers[0]);

	println!("\n==========================");
	println!("|| VECTOR GET LEN TESTS ||");
	println!("==========================");

	// You can find the length of the array
	println!("Vector length is: {}", numbers.len());

	println!("\n==============================");
	println!("|| VECTOR MEMORY SIZE TESTS ||");
	println!("==============================");

	// Memory size of the array on the STACK
	println!("Vector occupies {} bytes on the stack", std::mem::size_of_val(&numbers)); // '&' is used to reference the 'numbers' array in memory

	println!("\n=================");
	println!("|| SLICE TESTS ||");
	println!("=================");

	// Get Slice from the array
	//* &[i32] is a pointer to the data type ... 
	// = &numbers[0..2] is the size of the slice. index 0 to 1. the second number is always excluded
	let slice: &[i32] = &numbers[0..2];
	println!("Slice is: {:?}", slice);

	println!("\n=========================");
	println!("|| LOOP TROUGHT VALUES ||");
	println!("=========================");

	// Loop trought the Vector's values
	for i in numbers.iter() {
		println!("{}", i);
	}

	println!("\n============================");
	println!("|| LOOP AND MUTATE VALUES ||");
	println!("============================");

	println!("BEFORE: {:?}", numbers);

	// Loop trought and mutate Vector's values
	for i in numbers.iter_mut() {
		*i *= 2; // 'i' is a pointer. We access the value of the address it is pointing to by dereferencing it with '*'
	}
	println!("AFTER: {:?}", numbers);
}
