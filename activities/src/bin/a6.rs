// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut integer: u8 = 5;

    while integer > 0 {
        println!("{integer:?}");
        integer -= 1;
    }

    println!("done!");
}
