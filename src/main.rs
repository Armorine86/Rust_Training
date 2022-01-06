/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mmondell <mmondell@student.42quebec.com    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/06 11:59:20 by mmondell          #+#    #+#             */
/*   Updated: 2022/01/06 15:39:33 by mmondell         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::{self, Write};

mod args;
mod arrays;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod vectors;

enum Category {
    Empty,
    Print,
    Variables,
    Types,
    Strings,
    Tuples,
    Arrays,
    Vectors,
    Conditionals,
    Loops,
    Functions,
    Pointers,
    Structs,
    Enums,
    Args,
}

/// returns TRUE if input matches one of the Categories
fn valid_category(input: &str) -> Category {

    match input {
        "Print"         => return Category::Print,
        "Variables"     => return Category::Variables,
        "Types"         => return Category::Types,
        "Strings"       => return Category::Strings,
        "Tuples"        => return Category::Tuples,
        "Arrays"        => return Category::Arrays,
        "Vectors"       => return Category::Vectors,
        "Loops"         => return Category::Loops,
        "Functions"     => return Category::Functions,
        "Pointers"      => return Category::Pointers,
        "Structs"       => return Category::Structs,
        "Enums"         => return Category::Enums,
        "Args"          => return Category::Args,
        "Conditionals"  => return Category::Conditionals,
        _ => {
            println!("\n[{}] is not a valid category.", input);
            return Category::Empty;
        }
    }
}

fn usage() {
    
    println!("\n\nAvailable categories: \n
        [Print]
        [Variables]
        [Types]
        [Strings]
        [Tuples]
        [Arrays]
        [Vectors]
        [Conditionals]
        [Loops]
        [Functions]
        [Pointers]
        [Structs]
        [Enums]
        [Args] ")
}

/// Runs the Category matching user's selection
fn run_category_code(selection: Category) {
    match selection {
        Category::Empty => usage(),
        Category::Print => print::run("Hello from the other side"),
        Category::Variables => variables::run(),
        Category::Types => types::run(),
        Category::Strings => strings::run(),
        Category::Tuples => tuples::run(),
        Category::Arrays => arrays::run(),
        Category::Vectors => vectors::run(),
        Category::Conditionals => conditionals::run(),
        Category::Loops => loops::run(),
        Category::Functions => functions::run(),
        Category::Pointers => pointer_ref::run(),
        Category::Structs => structs::run(),
        Category::Enums => enums::run(),
        Category::Args => args::run(),
    }
}

fn uppercase_first_letter(input: &str) -> String {
    
   let mut c = input.chars();
   
   match c.next() {
       None => String::new(),
       Some(f) => f.to_uppercase().to_string() + c.as_str(),
   }
}
fn main() {
    
    loop {
        
        print!("\n\nChoose a Category: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input");
        let input = input.trim();
        let up_input = uppercase_first_letter(&input);

        if up_input == "Quit" {
              break;
        }
        
        let selection = valid_category(up_input.as_str());

        run_category_code(selection);
    }
}
