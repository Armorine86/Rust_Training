// ** ENUMS **

// Enums are types which have a few definit values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action

    //similar to switch
    match m {
        Movement::Up => println!("Moving UP!"),
        Movement::Down => println!("Moving DOWN!"),
        Movement::Left => println!("Moving LEFT!"),
        Movement::Right => println!("Moving RIGHT!"),
    }
}

pub fn run() {
    let up = Movement::Up;
    let down = Movement::Down;
    let left = Movement::Left;
    let right = Movement::Right;

    move_avatar(up);
    move_avatar(down);
    move_avatar(left);
    move_avatar(right);
}
