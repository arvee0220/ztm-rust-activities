// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User ID: {}, Name: {}", self.user_id, self.name)
    }
}

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let sam: String = String::from("Sam");

    let gpt: User = User {
        user_id: 4,
        name: String::from("GPT"),
    };


    let user = find_user(&sam).map(|user_id| User { user_id, name: sam });

    match user {
        Some(user) => println!("{user:?}"),
        None => println!("User not found"),
    }

    println!("{gpt:}")
}
