// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let result: i32 = add(60, 9);
    display_result(result);
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn display_result(res: i32){
    print!("The Result is {:?}", res);
}