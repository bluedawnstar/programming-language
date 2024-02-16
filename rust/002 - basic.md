# Rust

## 변수와 상수
  - 변수 선언
    ```Rust
    fn main() {
        let x = 5;
        println!("The value of x is: {}", x);
        //x = 6;  // error!
        let x = x + 1;
        println!("The value of x is: {}", x);
    }
    ```
    ```Rust
    fn main() {
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
    ```
    ```Rust
    fn main() {
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
    ```

  - 상수 선언
    ```Rust
    const MAX_POINTS: u32 = 100_000;

    fn main() {
        println!("MAX_POINTS = {}", MAX_POINTS);
    }
    ```

## 데이터 타입
  - 기본 타입
    - 정수 (integer)
      - 타입
        | Length    | Signed | Unsigned  |
        |:---------:|:------:|:---------:|
        | 8 bit     | i8     | u8        |
        | 16 bit    | i16    | u16       |
        | 32 bit    | i32    | u32       |
        | 64 bit    | i64    | u64       |
        | 128 bit   | i128   | u128      |
        | 32/64 bit | isize  | usize     |

      - 리터럴
        | Literals       | Example  |
        |:--------------:|:--------:|
        | Decimal        | 12_345   |
        | Hex            | 0x1234   |
        | Octal          | 0o123    |
        | Binary         | 0b10101  |
        | Byte (u8 only) | b'A'     |

    - 부동 소수점 (floating point)
      - 타입
        | Length    | type  |
        |:---------:|:-----:|
        | 32 bit    | f32   |
        | 64 bit    | f64   |

    - boolean
      - 타입
        | type  | false | true |
        |:-----:|:-----:|:----:|
        | bool  | false | true |

    - 문자
      - 타입
        | Length    | type   | literal          | 
        |:---------:|:------:|:----------------:|
        | 32 bit    | char   | 'A', '\u{1F600}' |

    - example
      ```Rust
      fn main() {
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
      ```
      ```Rust
      use std::mem;

      fn main() {
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
      ```

  - 복합 타입
    - 튜플 (tuple)
      ```Rust
      fn main() {
          let tup: (i32, f64, u8) = (500, 6.4, 1);
          println!("tup = {:?}", tup);

          let tup = (500, 6.4, 1);
          println!("({}, {}, {})", tup.0, tup.1, tup.2);    

          let (x, y, z) = tup;
          println!("x = {}, y = {}, z = {}", x, y, z);    
      }
      ```

    - 배열 (array)
      ```Rust
      fn main() {
          let a = [1, 2, 3, 4, 5];
          println!("a = {:?}, a[0] = {}, a[1] = {}", a, a[0], a[1]);

          let months = ["January", "February", "March", "April", "May", "June", "July",
                        "August", "September", "October", "November", "December"];
          println!("months = {:?}", months);
      }
      ```
      
## 함수 (function)
  - 기본 구조
    ```Rust
    fn main() {
        let x = plus(5, 3);

        println!("The value of x is: {}", x);
    }

    fn plus(x: i32, y: i32) -> i32 {
        x + y
    }
    ```
  