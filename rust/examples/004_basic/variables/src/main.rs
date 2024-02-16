// variables.rs

fn test1() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6;  // error!
    let x = x + 1;
    println!("The value of x is: {}", x);
}

fn test2() {
    let mut x = 7;
    println!("The value of x is: {}", x);
    x = 8;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

fn test3() {
    let mut x: i32 = 7;
    println!("The value of x is: {}", x);
    x = 8;
    println!("The value of x is: {}", x);
    let x: i32 = x + 1;
    println!("The value of x is: {}", x);

    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {}", spaces);
}


const MAX_POINTS: u32 = 100_000;

fn test_const() {
    println!("MAX_POINTS = {}", MAX_POINTS);
}

fn main() {
    test1();
    test2();
    test3();

    test_const();
}
