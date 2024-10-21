// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut integer: u8 = 1;

    loop {
        println!("{integer:?}");
        integer += 1;

        if integer  > 4 {
            break;
        }
    }
}
