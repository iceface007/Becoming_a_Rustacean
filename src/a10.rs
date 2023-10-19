// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print_mesasges(bool: bool) {
    match bool {
        true => println!("its big"),
        false => println!("its small")
    }
}

fn main() {
    let int: i32 = 101;
    let my_bool: bool = int > 100;
    print_mesasges(my_bool);
}
