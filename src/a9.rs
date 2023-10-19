// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print
fn get_coordinate() -> (i32, i32){
    let coords: (i32, i32) = (4, 7);
    coords
}

fn main() {
    let x: i32 = get_coordinate().0;
    let y: i32 = get_coordinate().1;
     
    if y > 5 {
        println!("Y({:?}) is greater than 5", y);
    } else if y == 5 {
        println!("Y({:?}) is equla to 5", y);    
    } else {
        println!("Y({:?}) is less than 5", y);
    }
    println!("The tuple is ({:?},{:?})", x, y);
}
