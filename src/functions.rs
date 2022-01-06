// ** FUNCTIONS **

// Functions - Used to store blocks of code for re-use

pub fn run() {
    println!("\n====================");
    println!("|| FUNCTION CALLS ||");
    println!("====================");

    // Function call
    greeting("Greetings to you", "John!");

    // Use a function call directly into another function
    println!("4 multiplied by 5 gives: {}", multiply(4, 5));

    // Bind a function to a variable
    let result = multiply(10, 15);
    println!("10 multiplied by 15 gives: {}", result);

    println!("\n======================");
    println!("|| CLOSURE FUNCTION ||");
    println!("======================");

    // Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    // Closures can capture values from the scope in which they’re defined.
    let num1: i32 = 50;

    let sum = |n1: i32, n2: i32| n1 + n2 + num1;
    println!("Closure sum: {}", sum(25, 25));
}

// greet: = the scope variable name and its type
// same thing for name
fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// We specify a return value with -> and the return type
fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2 // Omitting the semicolon here tells the Compiler that the result of this expression is the return value
}
