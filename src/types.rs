// ** PRIMITIVE TYPES **

// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of BITS they take in memory)
// ** 'u' is for Unsigned (cannot go negatve), 'i' is for integer (can be negative)

// Floats : f32, f64

// Booleans (bool)

// Characters (char)

// Tuples

// Arrays
// (arrays are 'fixed' length as opposed to Vectors which can be expanded or shrinked as needed)

// ** Rust is a "statically typed language", which means that it must know the types of all variables at compile time
// ** However, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    {
        println!("\n==============================");
        println!("|| PRIMITIVE VARIABLE TESTS ||");
        println!("==============================");

        let _unused: u32 = 32; // if you want to disable the "unused varaible" warning, put a '_' in front of the variable name

        // x will be an i32 int by default
        let x = 1;

        // y will be a f64 float by default
        let y = 42.5;

        // Add "EXPLICIT" type to a variable
        let z: i64 = 4242424242;

        //find max size of a variable
        println!(
            "Max i32: {}\nMax i64: {}\nMax f64: {}",
            std::i32::MAX,
            std::i64::MAX,
            std::f64::MAX
        );

        println!("\n=============================");
        println!("|| TRUE / FALSE BOOL TESTS ||");
        println!("=============================");

        // Boolean
        //let is_true = true;
        let is_true = false;

        if is_true {
            println!("Bool is true\n");
        } else {
            println!("Bool is false\n");
        }
        println!("{:?}", (x, y, z, is_true))
    }

    {
        println!("\n================================");
        println!("|| BOOL FROM EXPRESSION TESTS ||");
        println!("================================");

        let (x, y) = (10, 5);

        let x_is_greater: bool = x > y;

        if x_is_greater {
            println!("{}[x] is greater than {}[y]", x, y)
        } else {
            println!("{}[x] is smaller than {}[y]", x, y)
        }
    }

    {
        println!("\n================");
        println!("|| CHAR TESTS ||");
        println!("================");

        let chr = 'A'; // Characters are assigned with single quotes

        let smiley = '\u{1F600}'; // \u is for unicode and the value between curly braces is for a smiley face emoji

        println!("Simple char: '{}'\nUnicode emoji: {}", chr, smiley);
    }
}
