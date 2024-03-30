fn main() {
    let mut x = 316;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let y = 10;
    let y = y + 2;
    {
        let y = y * 2;
        println!("The value of y in the inner scoper is: {y}")
    }
    println!("The value of x is: {y}")
}
