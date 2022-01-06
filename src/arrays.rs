// Arrays - Fixed list where elements are all the same data type

pub fn run() {
    println!("\n================================");
    println!("|| ARRAYS INITILIZATION TESTS ||");
    println!("================================");

    // [i32; 5] --> [type; length] ... = [element, element, element]
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // If you remove an element from the array, you will have a compilation error.
    // The length of the array is 'FIXED' and cannot be modified.
    // You can however modify the value inside the array as long as they remain of the same type

    println!("\n============================");
    println!("|| ARRAYS RE-ASSIGN TESTS ||");
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

    println!("\n==============================");
    println!("|| ACCESS ARRAY INDEX TESTS ||");
    println!("==============================");

    // Access a single value in the array
    println!("Value at index [0] is: {}", numbers[0]);

    println!("\n==========================");
    println!("|| ARRAYS GET LEN TESTS ||");
    println!("==========================");

    // You can find the length of the array
    println!("Array length is: {}", numbers.len());

    println!("\n==============================");
    println!("|| ARRAYS MEMORY SIZE TESTS ||");
    println!("==============================");

    // Memory size of the array on the STACK
    println!(
        "Array occupies {} bytes on the stack",
        std::mem::size_of_val(&numbers)
    ); // '&' is used to reference the 'numbers' array in memory

    println!("\n=================");
    println!("|| SLICE TESTS ||");
    println!("=================");

    // Get Slice from the array
    //* &[i32] is a pointer to the data type ...
    // = &numbers[0..2] is the size of the slice. index 0 to 1. the second number is always excluded
    let slice: &[i32] = &numbers[0..2];
    println!("Slice is: {:?}", slice);
}
