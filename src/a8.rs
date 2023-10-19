// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Drinkflavor {
    Sweet,
    Salty,
    Yummy,
}

struct Drink {
    flavor: Drinkflavor,
    fluid_ounce: f64,
}

fn print_drink(drink: Drink){

    println!("fluid ounce: {:?}", drink.fluid_ounce,);

    match drink.flavor {
        Drinkflavor::Sweet => println!("flavor: So sweet!"),
        Drinkflavor::Salty => println!("flavor: Salty <3"),
        Drinkflavor::Yummy => println!("flavor: Yummy so good!"),
    }
}
fn main() {
    let my_drink = Drink{
        flavor: Drinkflavor::Sweet,
        fluid_ounce: 3.5,
    };

    print_drink(my_drink);
}
