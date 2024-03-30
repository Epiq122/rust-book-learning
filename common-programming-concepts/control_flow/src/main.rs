fn main() {
    //if
    let age = 18;
    if age < 18 {
        println!("your not old enough to vote")
    } else {
        println!("you can vote")
    }

    // not equal
    let number = 316;

    if number != 0 {
        println!("number was something other than 0")
    }

    // if else

    let points = 10;

    if points >= 100 {
        println!("You have somehow got a perfect score holy smokes");
    } else if points >= 75 {
        println!("Great job! You're doing well.");
    } else if points >= 50 {
        println!("Heh very very mid");
    } else if points >= 25 {
        println!("This isn't good oh boy");
    } else {
        println!("Not worth the time sorry");
    }

    // using if in a let statement
    let condition = true;
    let nums = if condition { 5 } else { 6 };
    println!("THe value of nums is : {nums}");

    // repeating code with loops
    // loop {
    //     println!("again!")
    // }

    // return values from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10000 {
            break counter * 2;
        }
    };
    println!("the result is {result}");
    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while

    let mut numba = 3;
    while numba != 0 {
        println!("{numba}!");
        numba -= 1;
    }
    println!("WE HAVE LIFTOFF!");

    // looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // using for
    for element in a {
        println!("the value is: {element}")
    }

    let teams = ["oilers", "canucks", "sharks", "ducks", "blackhawks"];
    for team in teams {
        println!("Western Conference: {team}")
    }

    // using reverse
    for numbero in (1..10).rev() {
        println!("{numbero}")
    }
    println!("LIFTOFF!")
}
