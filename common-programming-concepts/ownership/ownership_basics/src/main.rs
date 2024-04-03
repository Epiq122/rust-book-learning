fn main() {
    // variable scope
    let mut team = String::from("Edmonton");
    team.push_str(" Oilers"); // add to the end of the string
    println!("{team}");

    // varaibles and data interacting with Move
    let x = 316;
    let y = x;
    println!("X: {x}, Y: {y}");

    let s1 = String::from("Epiq");
    let s2 = s1;
    // println!("S1: {s1}, S2: {s2}"); // ownership errror

    // varaibnles and data interacting with clone

    let s3 = String::from("Exitus");
    let s4 = s3.clone();
    println!("s3: {s3}, s4: {s4}");

    // ownership and functions

    let s = String::from("Pink Floyd"); // s comes into scope
    takes_ownership(s); // s value moves into the function and is no longer valid here
                        // println!("{s}"); // borrow of moved value: `s

    let x = 5;
    makes_copy(x);
    println!("{x}");

    // return values and Scope

    let s5 = gives_ownership();
    println!("{s5}");

    let s6 = String::from("Hillers");
    let s7 = takes_and_gives_back(s6);
    println!("{s7}")
}

// ownership and functions

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}")
} // some_string goes out of scope and drop is called. The backing memory is freed.

fn makes_copy(some_int: i32) {
    // some int comes into scope
    println!("{some_int}");
} // some int goes out of scope nothing happens

// return values and scope
fn gives_ownership() -> String {
    let some_string = String::from("Hankers");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
