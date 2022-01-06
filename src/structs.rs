// ** STRUCTS **

// Structs - Used to create custom data types (Similar to Classes)

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Rgb(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // ** THESE FUNCTIONS ARE CALLED METHODS

    // Construct
    fn new(name: &str, last: &str) -> Person {
        Person {
            first_name: name.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name
    //* &self is a reference to the struct. In this case, Person
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!(
        "Traditionnal struct --> [R] [G] [B]: [{}] [{}] [{}]",
        c.red, c.green, c.blue
    );

    c.red = 200;
    c.green = 100;
    c.blue = 50;

    println!(
        "Traditionnal struct --> [R] [G] [B]: [{}] [{}] [{}]",
        c.red, c.green, c.blue
    );

    let mut rgb = Rgb(255, 0, 0);

    println!(
        "\nTuple Struct        --> [R] [G] [B]: [{}] [{}] [{}]",
        rgb.0, rgb.1, rgb.2
    );

    rgb.0 = 150;
    rgb.1 = 125;
    rgb.2 = 25;

    println!(
        "Tuple Struct        --> [R] [G] [B]: [{}] [{}] [{}]",
        rgb.0, rgb.1, rgb.2
    );

    let mut john = Person::new("John", "Carpenter");

    println!("\nName: {}\nLast name: {}", john.first_name, john.last_name);
    println!("\nPerson: {}", john.full_name());

    john.set_last_name("Doe");

    println!("\nPerson: {}", john.full_name());

    println!("\nPerson Tuple: {:?}", john.to_tuple());
}
