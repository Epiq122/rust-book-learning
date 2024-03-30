fn main() {
    println!("Welcome to my");
    another_function();
    function_with_param(316);
    print_labeled_measurement(5, 'h');

    // calling a macro
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // return
    let a = eight();
    println!("The value of x is: {a}");

    let x = plus_one(315);
    println!("The value of x is: {x}")
}

fn another_function() {
    println!("Nightmare!")
}

fn function_with_param(x: i32) {
    println!("Austin {x} says i just whooped your !")
}

// multiple params
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// return values

fn eight() -> i32 {
    8
}

fn plus_one(j: i32) -> i32 {
    j + 1
}
