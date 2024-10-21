// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn sum(num1: i32, num2: i32) -> i32 {
   return num1 + num2;
}
// * Use a function to display the result
fn display_result(result: i32) {
    println!("{result:?}");
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display_result(sum(5,5))
}
