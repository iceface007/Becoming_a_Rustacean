// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let int = 5;

    match int {
        1 => print!("one"),
        2 => print!("two"),
        3 => print!("three"),
        _ => print!("some other number"),
    }
}
