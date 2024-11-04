// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut item = HashMap::new();

    item.insert("Chairs", 5);
    item.insert("Beds", 3);
    item.insert("Tables", 2);
    item.insert("Couches", 0);

    let mut total_qty = 0;

    println!("Available furnitures: ");

    for (name, quantity) in item.iter() {
        total_qty = total_qty + quantity;

        if *quantity > 0 {
            println!("name: {name:}");
            println!("Quantity: {quantity:}");
        } else {
            println!("{name:}, out of stock");
        }
    }

    println!("Total quantity: {total_qty:}");

    total_qty = 0;

    for (name, quantity) in item.iter() {
        total_qty = total_qty + quantity;
        match quantity {
            0 => println!("{name}, out of stock"),
            _ => print!("(name: {name}, quantity: {quantity}), "),
        }
    }

    println!("Total quantity: {total_qty:}");
}
