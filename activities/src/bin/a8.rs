// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavor {
    Lychee,
    Kiwi,
    Strawberry,
    CucumberLemon,
}

struct Drink {
    flavor: DrinkFlavor,
    fluid_oz: u8,
}

fn drink_details(drink: Drink) {
    let flavor: DrinkFlavor = drink.flavor;
    let fluid_oz: u8 = drink.fluid_oz;

    match flavor {
        DrinkFlavor::Lychee => println!("Flavor: Lychee"),
        DrinkFlavor::Kiwi => println!("Flavor: Kiwi"),
        DrinkFlavor::Strawberry => println!("Flavor: Strawberry"),
        DrinkFlavor::CucumberLemon => println!("Flavor: Cucumber Lemon"),
    }

    println!("Fluid oz: {fluid_oz:?}");
}

fn main() {
    let blue: Drink = Drink {
        flavor: DrinkFlavor::Lychee,
        fluid_oz: 21,
    };

    let sola: Drink = Drink {
        flavor: DrinkFlavor::Kiwi,
        fluid_oz: 21,
    };

    let tang: Drink = Drink {
        flavor: DrinkFlavor::CucumberLemon,
        fluid_oz: 21,
    };

    let dr_pepper: Drink = Drink {
        flavor: DrinkFlavor::Strawberry,
        fluid_oz: 21,
    };

    drink_details(blue);
    drink_details(sola);
    drink_details(tang);
    drink_details(dr_pepper);
}
