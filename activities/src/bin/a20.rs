// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io::stdin;
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn power_message(input: PowerState) {
    match input {
        PowerState::Off => println!("Device is currently disconnected"),
        PowerState::Sleep => println!("Initializing sleep mode"),
        PowerState::Reboot => println!("Initializing reboot"),
        PowerState::Shutdown => println!("Device is shutting down"),
        PowerState::Hibernate => println!("Device is hibernating"),
    }
}

fn main() {
    loop {
        println!("Please enter a command (e.g., off, sleep, reboot, shutdown, hibernate)");

        let mut input: String = String::new();

        stdin().read_line(&mut input).expect("Failed to readline");

        let input = input.trim().to_lowercase();

        if input == "exit" {
            println!("Exiting program.");
            break;
        }

        match input.as_str() {
            "off" => power_message(PowerState::Off),
            "sleep" => power_message(PowerState::Sleep),
            "reboot" => power_message(PowerState::Reboot),
            "shutdown" => power_message(PowerState::Shutdown),
            "hibernate" => power_message(PowerState::Hibernate),
            _ => println!("Unknown command"),
        }
    }
}
