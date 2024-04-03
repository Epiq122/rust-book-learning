fn main() {
    let s1 = String::from("Epiq");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");

    // mutable reference
    let mut s = String::from("Edmonton");
    change(&mut s);
    println!("{s}");

    let s1 = String::from("Epiq");
    let r1 = &s1;
    let r2 = &s1;
    println!("{r1}, {r2}");
    // r1 abd r2 will not be used after this point
}

fn calculate_length(s: &String) -> usize {
    // s is a refrence to a string
    s.len()
} // s goes out of scope. But it does not have ownership of what it referes to, it is not dropped

// Mutable Reference
fn change(some_string: &mut String) {
    some_string.push_str(" Oilers");
}
