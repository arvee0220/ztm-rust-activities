// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    White,
    Black,
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    height: f64,
    length: f64,
    width: f64,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(h: f64, l: f64, w: f64, wt: f64, clr: Color) -> Self {
        Self {
            height: h,
            length: l,
            width: w,
            weight: wt,
            color: clr,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print_characteristics(&self) {
        println!(
            "Height: {:}in, Length: {:}in, Width: {:}in, Weight: {:}lbs, Color: {:?}",
            self.height, self.length, self.width, self.weight, self.color
        );
    }
}

fn main() {
    let black_box: ShippingBox = ShippingBox::new(10.0, 10.0, 10.0, 32.5, Color::White);
    let white_box: ShippingBox = ShippingBox::new(20.0, 20.0, 20.0, 64.1, Color::Black);

    black_box.print_characteristics();
    white_box.print_characteristics();
}
