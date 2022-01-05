// |===========================|
// |SOME IMPORTANT INFORMATIONS|
// |===========================|

// 1. Variables hold primitives data or references to data
// 2. Variables are immutable by default (meaning you cannot re-assign the same variable)
// 3. Rust is a block-scoped language

pub fn run() {

	{
		println!("\n===================================");
		println!("||  SIMPLE VARIABLE PLACEHOLDER  ||");
		println!("===================================");
		//This will work
		let name = "Maxime";
		let age = 36;
	
		println!("\nMy name is {} and I am {}", name, age);
	}
	
	//* {
	//* 	let name = "Maxime";
	//?		let age = 36;
	
	//* 	//BIRTHDAY !!!!

	//? 	age = 37;	** Cannot assign twice to immutable variable 'age'
	//? 				** We CAN'T reassign an immutable variable after it has been declared
	//? 				** Equivalent to 'const'
				

	// 	println!("\nMy name is {} and I am {}", name, age);
	// }

	{
		println!("\n=============================");
		println!("||  MUTABLE VARIABLE TEST  ||");
		println!("=============================");
		let name = "Maxime";
		let mut age = 36;	// Adding the 'mut' Keyword make the variable 'mutable'
							// So we can reassign a new value to it

		// you MUST use the variable before it is modified or you get a compilation warning
		println!("\nMy name is {} and I am {}", name, age);
		
		//BIRTHDAY !!!!

		age = 37;
		println!("\nMy name is {} and I am {}", name, age);
	}

	{
		println!("\n============================");
		println!("|| CONSTANT VARIABLE TEST ||");
		println!("============================");

		// By convention, const variables are all UPPERCASE and spaces with '_'
		const LANGUAGE: &str = "Rust"; // you must specify a type with a :<type> after the variable name

		println!("{} is my language", LANGUAGE);
	}

	{
		println!("\n===================================");
		println!("|| MULTIPLE VARIABLE ASSIGNATION ||");
		println!("===================================");

		let (name, age) = ("Maxime", 36);	// name all the variable in between parenthesis
											// and assign them in the same way

		println!("My name is: {} and I am {} years old", name, age);
	}
}
