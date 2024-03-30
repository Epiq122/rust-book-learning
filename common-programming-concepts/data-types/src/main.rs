fn main() {
    // floats
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32
    println!("float 64 {}, float32 {}", x, y);

    //numeric operators
    let sum = 5 + 10;
    let difference = 95.5 - 10.2;
    let product = 10 * 10;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Truncated: {}", truncated);
    println!("Remainder: {}", remainder);

    // bool
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // char
    let l = 'z';
    let x: char = 'Z';
    let beer = '🍺';

    println!("l: {}", l);
    println!("x: {}", x);
    println!("beer: {}", beer);

    // tuple

    let tup: (i32, f64, u8) = (316, 8.2, 4);
    println!("Tuple: {:?}", tup);

    // another way
    let tup2 = (100, 1.1, 2);
    let (_f, _g, h) = tup2;
    println!("The value of h is: {h}");

    // accessing a element
    let r: (i32, f64, bool) = (88, 3.16, true);
    let eighty_eight = r.0;
    let three_point_one_six = r.1;
    let has_car = r.2;

    println!("Value of eighty_eight: {}", eighty_eight);
    println!("Value of three_point_one_six: {}", three_point_one_six);
    println!("Value of has_car: {}", has_car);

    // array
    let array = [1, 2, 3, 4, 5, 6, 7];
    println!("Array: {:?}", array);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let first_month = months[0];
    let last_month = months[11];

    println!("First month: {}", first_month);
    println!("Last month: {}", last_month);
}
