// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: u8,
    name: String,           // * The color and name should be stored as a String
    favorite_color: String, // * The color and name should be stored as a String
}

/* impl Person {
    fn print_name(&self) {
        println!("{:}", self.name)
    }

    fn print_color(&self) {
        println!("{:}", self.favorite_color)
    }
} */

fn print(data: &str) {
    println!("{data:}")
}

fn main() {
    let people: Vec<Person> = vec![
        Person {
            age: 21,
            name: String::from("Sky"),
            favorite_color: String::from("Blue"),
        },
        Person {
            age: 10,
            name: String::from("Heart"),
            favorite_color: String::from("Pink"),
        },
        Person {
            age: 25,
            name: String::from("Chen"),
            favorite_color: String::from("Violet"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in &people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.favorite_color);
        }
    }
}
