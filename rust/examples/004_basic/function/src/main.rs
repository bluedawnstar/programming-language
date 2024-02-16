fn main() {
    let x = plus(5, 3);

    println!("The value of x is: {}", x);
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}
