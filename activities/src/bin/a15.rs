// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
// * Tickets can be Backstage, Vip, and Standard
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let diddy_concert: Vec<Ticket> = vec![
        Ticket::Backstage(15000.0, String::from("P.Diddy")),
        Ticket::Vip(10000.00, String::from("Beyonce")),
        Ticket::Standard(5000.00),
    ];

    for ticket in diddy_concert {
        // * Backstage and Vip tickets include the ticket holder's name
        match ticket {
            Ticket::Vip(price, name_of_holder) => println!(
                "Name of holder: {:}, Ticket Price: {:}",
                name_of_holder, price
            ),
            Ticket::Backstage(price, name_of_holder) => println!(
                "Name of holder: {:}, Ticket Price: {:}",
                name_of_holder, price
            ),
            Ticket::Standard(price) => println!("Ticket Price: {:}", price),
        }
    }
}
