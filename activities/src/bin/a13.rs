// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    // * Use a vector to store 4 numbers
    let my_vec: Vec<i64> = vec![10, 20, 30, 40];

    // * Iterate through the vector using a for..in loop
    for i in &my_vec {
        // * Determine whether to print the number or print "thirty" inside the loop
        if *i == 30 {
            println!("thirty")
        } else {
            println!("{i:}")
        }
    }

    let vec_len = my_vec.len();

    // * Use the .len() function to print the number of elements in a vector
    println!("my_vec length is {vec_len:}")
}
