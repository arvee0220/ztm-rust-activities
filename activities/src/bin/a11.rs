// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_qty(grocery_item: &GroceryItem) {
    let quantity = grocery_item.quantity;

    println!("Quantity: {quantity:}");
}

fn display_id(grocery_item: &GroceryItem) {
    let id = grocery_item.id;

    println!("Id: {id:}");
}
fn main() {
    let cereal: GroceryItem = GroceryItem {
        quantity: 2,
        id: 1,
    };

    display_qty(&cereal);
    display_id(&cereal);
}
