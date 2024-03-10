fn main() {
    println!("Hello world");

    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let mut name = "Rohit";
    println!("Hi {}", name);

    name = "Rohit Kumar";
    println!("Hi again you changed your name to  {}", name);

    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let d = 'r'; // unicode character
    let ferris = '🦀'; // also a unicode character
    let bv = true;
    let t = (13, false); //tuple
    let arr = ["rohit", "mohit"];
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {}",
        x,
        a,
        b,
        c,
        d,
        ferris,
        bv,
        t.0,
        t.1,
        sentence,
        arr[0],
        arr[1] //12 12 4.3 4.3 r 🦀 true 13 false hello world!
    );

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", !t as u8);

    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    let sum = add_two_numbers(1, 2);
    println!("{}", sum);

    let sum = subtract(1, 2);
    println!("{}", sum);

    let mut x = 5;
    {
        // creating a mutable reference to x
        let y = &mut x;
        // dereferencing y to access the value, and then
        // assigning a new value to it
        *y = 10;
    }
    // printing the final value of x, which is now 10
    println!("{}", x);

    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);

    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("x is {}", x);

    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }

    switch_case();

    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13"; // break with a value
        }
    };
    println!("from loop: {}", v);

    println!("from function: {}", example());

    // Using a static method to create an instance of String
    let s = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn switch_case() -> () {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}

fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4
}
