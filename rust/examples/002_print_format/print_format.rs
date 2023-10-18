fn main() {
    print!("Hello, world!\n");  // io::stdout
    println!("Hello, world!");  // io::stdout

    eprint!("(stderr) Hello, world!\n"); // io::stderr
    eprintln!("(stderr) Hello, world!"); // io::stderr

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{}, {:b}, {:o}, {:x}, {:X}", 26, 26, 26, 26, 26);
    println!("{:>10}, {:>10b}, {:>10o}, {:>10x}, {:>10X}", 26, 26, 26, 26, 26);
    println!("{:0>10}, {:0>10b}, {:0>10o}, {:0>10x}, {:0>10X}", 26, 26, 26, 26, 26);
    println!("{:<10}, {:<10b}, {:<10o}, {:<10x}, {:<10X}", 26, 26, 26, 26, 26);
    println!("{:-<10}, {:-<10b}, {:-<10o}, {:-<10x}, {:-<10X}", 26, 26, 26, 26, 26);
    println!("{:>6}, {:>6b}, {:>6o}, {:>6x}, {:>6X}", 123456, 123456, 123456, 123456, 123456);
    println!("{:0>6}, {:0>6b}, {:0>6o}, {:0>6x}, {:0>6X}", 123456, 123456, 123456, 123456, 123456);
    println!("{}, {}, {}, {}, {}, {}", 26, 026, 0b11010, 0o32, 0x1a, 0x1A);
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);

    let pi = 22.0/7.0;
    println!("Pi is roughly {0:.3}", pi);
    println!("Pi is roughly {pi:.3}", pi=pi);
    println!("Pi is roughly {0:.prec$}", pi, prec=4);
    println!("Pi is roughly {pi:.prec$}", pi=pi, prec=4);
    println!("Pi is roughly {pi:.prec$}", prec=5, pi=pi);

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    println!("{number:0>width$}");
}
