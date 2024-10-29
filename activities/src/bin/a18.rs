// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: u32,
}

fn restricted_purchase(customer: &Customer) -> Result<String, String> {
    if customer.age >= 21 {
        println!("{:?}", customer.name);
        Ok(String::from("Not a minor"))
    } else {
        Err(String::from("Minor"))
    }
}

fn main() {
    let buboy: Customer = Customer {
        name: String::from("Buboy"),
        age: 18,
    };

    let purchase = restricted_purchase(&buboy);

    println!("{purchase:?}")
}
