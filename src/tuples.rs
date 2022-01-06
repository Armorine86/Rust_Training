// ** TUPLES **

// 1. Tuples are group of values with different types

// 12 elements MAXIMUM

pub fn run() {
    println!("\n================");
    println!("|| TUPLE TEST ||");
    println!("================");

    // maxime contains a bunch of different types which makes is a 'tuple' type
    let maxime: (&str, &str, i32, f64) = ("Maxime", "Mondello", 36, 5.6);

    // maxime.0..1..2 is how you access the index of the tuple
    println!(
        "{} {} is {} years old and {} feet tall",
        maxime.0, maxime.1, maxime.2, maxime.3
    );
}
