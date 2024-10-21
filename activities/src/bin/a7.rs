// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Black,
    White,
    Blue,
    Red,
}

fn color_name(color: Color) {
    match color {
        Color::Black => println!("Black"),
        Color::White => println!("White"),
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
    }
}

fn main() {
    let black: Color = Color::Black;
    let white: Color = Color::White;
    let blue: Color = Color::Blue;
    let red: Color = Color::Red;

    color_name(black);
    color_name(white);
    color_name(blue);
    color_name(red);
}
