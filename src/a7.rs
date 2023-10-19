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

enum Colors {
    Red, 
    Green,
    Blue,
    Yellow,
}

fn print_color_name(colors: Colors){

    match colors {
        Colors::Blue => println!("Blue"),
        Colors::Green => println!("Green"),
        Colors::Red => println!("Red"),
        Colors::Yellow => println!("Yellow"),
    }
}

fn main() {
    print_color_name(Colors:: Red);
}


