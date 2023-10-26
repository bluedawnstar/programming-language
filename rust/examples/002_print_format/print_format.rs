// https://doc.rust-lang.org/std/fmt/index.html

fn main() {
    print!("Hello, world!\n");              // "Hello, world!"
    println!("Hello, world!");              // "Hello, world!"

    eprint!("(stderr) Hello, world!\n");    // "(stderr) Hello, world!"
    eprintln!("(stderr) Hello, world!");    // "(stderr) Hello, world!"

    println!("{} days", 31);                // "31 days"
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
                                            // "Alice, this is Bob. Bob, this is Alice"
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");            // "the quick brown fox jumps over the lazy dog"
    println!("My name is {0}, {1} {0}", "Bond", "James");
                                            // "My name is Bond, James Bond"

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
                                            // "1 of 10 people know binary, the other half doesn't"
    println!("{}, {:b}, {:o}, {:x}, {:X}", 26, 26, 26, 26, 26);
                                            // "26, 11010, 32, 1a, 1A"
    println!("{:>10}, {:>10b}, {:>10o}, {:>10x}, {:>10X}", 26, 26, 26, 26, 26);
                                            // "        26,      11010,         32,         1a,         1A"
    println!("{:0>10}, {:0>10b}, {:0>10o}, {:0>10x}, {:0>10X}", 26, 26, 26, 26, 26);
                                            // "0000000026, 0000011010, 0000000032, 000000001a, 000000001A"
    println!("{:<10}, {:<10b}, {:<10o}, {:<10x}, {:<10X}", 26, 26, 26, 26, 26);
                                            // "26        , 11010     , 32        , 1a        , 1A        "
    println!("{:-<10}, {:-<10b}, {:-<10o}, {:-<10x}, {:-<10X}", 26, 26, 26, 26, 26);
                                            // "26--------, 11010-----, 32--------, 1a--------, 1A--------"
    println!("{:>6}, {:>6b}, {:>6o}, {:>6x}, {:>6X}", 123456, 123456, 123456, 123456, 123456);
                                            // "123456, 11110001001000000, 361100,  1e240,  1E240"
    println!("{:0>6}, {:0>6b}, {:0>6o}, {:0>6x}, {:0>6X}", 123456, 123456, 123456, 123456, 123456);
                                            // "123456, 11110001001000000, 361100, 01e240, 01E240"
    println!("{}, {}, {}, {}, {}, {}", 26, 026, 0b11010, 0o32, 0x1a, 0x1A);
                                            // "26, 26, 26, 26, 26, 26"
    println!("{number:>5}", number=1);                  // "    1"
    println!("{number:0>5}", number=1);                 // "00001"
    println!("{number:>width$}", number=1, width=6);    // "     1"
    println!("{number:0>width$}", number=1, width=6);   // "000001"

    let pi = 22.0/7.0;
    println!("Pi is roughly {0:.3}", pi);                 // "Pi is roughly 3.143"
    println!("Pi is roughly {pi:.3}", pi=pi);             // "Pi is roughly 3.143"
    println!("Pi is roughly {0:.prec$}", pi, prec=4);     // "Pi is roughly 3.1429"
    println!("Pi is roughly {pi:.prec$}", pi=pi, prec=4); // "Pi is roughly 3.1429"
    println!("Pi is roughly {pi:.prec$}", prec=5, pi=pi); // "Pi is roughly 3.14286"

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");                       // "    1"
    println!("{number:0>width$}");                      // "00001"
}
