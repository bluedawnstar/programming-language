use std::mem;

fn integer_types() {
    let xi8 = 0b1111_0000u8 as i8;
    let xu8 = 0b1111_0000u8;
    let xi16 = 0xff00u16 as i16;
    let xu16 = 0xff00u16;
    let xi32 = 0xff00_ff00u32 as i32;
    let xu32 = 0xff00_ff00u32;
    let xi64 = 0xff00_ff00_ff00_ff00u64 as i64;
    let xu64 = 0xff00_ff00_ff00_ff00u64;
    let xi128 = -0xff00_ff00_ff00_ff00_ff00_ff00_ff00i128;
    let xu128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00u128;
    let xisize = 0xff00_ff00_ff00_ff00usize as isize;
    let xusize = 0xff00_ff00_ff00_ff00usize;

    println!("xi8 = {}, {:b}, {:o}, {:x}", xi8, xi8, xi8, xi8);
    println!("xu8 = {}, {:b}, {:o}, {:x}", xu8, xu8, xu8, xu8);
    println!("xi16 = {}, {:b}, {:o}, {:x}", xi16, xi16, xi16, xi16);
    println!("xu16 = {}, {:b}, {:o}, {:x}", xu16, xu16, xu16, xu16);
    println!("xi32 = {}, {:b}, {:o}, {:x}", xi32, xi32, xi32, xi32);
    println!("xu32 = {}, {:b}, {:o}, {:x}", xu32, xu32, xu32, xu32);
    println!("xi64 = {}, {:b}, {:o}, {:x}", xi64, xi64, xi64, xi64);
    println!("xu64 = {}, {:b}, {:o}, {:x}", xu64, xu64, xu64, xu64);
    println!("xi128 = {}, {:b}, {:o}, {:x}", xi128, xi128, xi128, xi128);
    println!("xu128 = {}, {:b}, {:o}, {:x}", xu128, xu128, xu128, xu128);
    println!("xisize = {}, {:b}, {:o}, {:x}", xisize, xisize, xisize, xisize);
    println!("xusize = {}, {:b}, {:o}, {:x}", xusize, xusize, xusize, xusize);
}

fn charactor_types() {
    let ch = 'A';
    println!("ch = {}, size={}", ch,  mem::size_of_val(&ch));
    println!("ch = {}", ch as u32);
    let ch8 = b'A';
    println!("ch8 = {}, size={}", ch8,  mem::size_of_val(&ch8));
    println!("ch8 = {}", ch8 as char);

    let ch = '\u{1F600}';
    println!("ch = {}, size={}", ch,  mem::size_of_val(&ch));
    println!("ch = {}, {:b}, {:o}, {:x}", ch as i8, ch as i8, ch as i8, ch as i8);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as u8, ch as u8, ch as u8, ch as u8);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as i16, ch as i16, ch as i16, ch as i16);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as u16, ch as u16, ch as u16, ch as u16);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as i32, ch as i32, ch as i32, ch as i32);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as u32, ch as u32, ch as u32, ch as u32);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as i64, ch as i64, ch as i64, ch as i64);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as u64, ch as u64, ch as u64, ch as u64);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as i128, ch as i128, ch as i128, ch as i128);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as u128, ch as u128, ch as u128, ch as u128);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as isize, ch as isize, ch as isize, ch as isize);
    println!("ch = {}, {:b}, {:o}, {:x}", ch as usize, ch as usize, ch as usize, ch as usize);
    println!("ch = {}", ch as u32 as f32);
    println!("ch = {}", ch as u32 as f64);
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);

    let tup = (500, 6.4, 1);
    println!("({}, {}, {})", tup.0, tup.1, tup.2);    

    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);    
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}, a[0] = {}, a[1] = {}", a, a[0], a[1]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("months = {:?}", months);
}

fn main() {
    integer_types();
    charactor_types();
    tuple_types();
    array_types();
}
