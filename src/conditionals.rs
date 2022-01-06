// ** CONDITIONALS **

// Conditionals - Used to check the condition of an expression and act on the result

pub fn run() {
    {
        println!("\n=======================================");
        println!("|| BASIC IF / ELSE CONDITIONAL TESTS ||");
        println!("=======================================");

        let mut age = 18;

        // If / Else conditionals
        if age >= 21 {
            println!("Bartender: What would you want to drink?");
        } else {
            println!("Bartender: Sorry you have to leave"); // Will print this part since the Conditional result is False
        }

        age = 25;

        if age >= 21 {
            println!("Bartender: What would you want to drink?"); // Will print this part since the Conditional result is True
        } else {
            println!("Bartender: Sorry you have to leave");
        }
    }

    {
        println!("\n==================================================================");
        println!("|| BASIC IF / ELSE IF / ELSE CONDITIONAL TESTS WITH && OPERATOR ||");
        println!("==================================================================");

        let mut age = 18;
        let mut check_id: bool = false;

        // the && operator means that BOTH expression must be true to execute the command
        if age >= 21 && check_id {
            println!("Bartender: What would you want to drink?");
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID"); // age AND check_id are both false so this part is printed
        }

        age = 21;

        // BOTH expression must evaluate to TRUE to execute the next step. Otherwise, the program jumps to the next conditional
        if age >= 21 && check_id {
            println!("Bartender: What would you want to drink?");
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID"); // age evaluates to true but check_id is still false so this part is printed
        }

        check_id = true;

        if age >= 21 && check_id {
            println!("Bartender: What would you want to drink?"); // age and check_id BOTH evaluates to TRUE so this part is printed
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID");
        }

        age = 16;

        if age >= 21 && check_id {
            println!("Bartender: What would you want to drink?");
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave"); // age and check_id BOTH evaluates to true so this part is printed
        } else {
            println!("Bartender: Show me your ID");
        }
    }

    {
        println!("\n==================================================================");
        println!("|| BASIC IF / ELSE IF / ELSE CONDITIONAL TESTS WITH || OPERATOR ||");
        println!("==================================================================");

        let mut age = 18;
        let mut check_id: bool = false;
        let knows_person: bool = true;

        // the || operator means only one of the expressions needs to evaluate to TRUE to execute the next command
        if age >= 21 && check_id || knows_person {
            println!("Bartender: What would you want to drink?"); // This part will be printed because knows_person is true
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID");
        }

        age = 21;

        if age >= 21 && check_id || knows_person {
            println!("Bartender: What would you want to drink?"); // in every scenario
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID");
        }

        check_id = true;

        if age >= 21 && check_id || knows_person {
            println!("Bartender: What would you want to drink?");
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID");
        }

        age = 16;

        if age >= 21 && check_id || knows_person {
            println!("Bartender: What would you want to drink?");
        } else if age < 21 && check_id {
            println!("Bartender: Sorry you have to leave");
        } else {
            println!("Bartender: Show me your ID");
        }
    }

    {
        println!("\n==================");
        println!("|| SHORTHAND IF ||");
        println!("==================");

        let mut age = 18;
        let mut is_of_age = if age >= 21 { true } else { false };

        println! {"The person is of age: {}", is_of_age};

        age = 22;
        is_of_age = if age >= 21 { true } else { false };
        println! {"The person is of age: {}", is_of_age};
    }
}
