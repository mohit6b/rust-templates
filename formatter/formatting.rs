fn main() {

    println!("Rust is safe, fast and concurrent. It is memory safe without garbage collection");

    /* COMMENTS */
    // Line comments
    println!("Above is a line comment");

    /*
    Block
    */
    println!("Above is a block comment");

    println!("/// Library doc - following item -> generates library doc for the following item");


    println!("//! Library doc - enclosed item -> generates libraray doc for the enclosed item");

    let a = 5 + /* 90 +  */ 5; 
    println!("Is a 10 or 100. a = {}", a);

    println!("{} days", 31);

    println!("{subject} {verb} {object}", 
            subject = "The quick brown fox",
            object = "the lazy dog",
            verb = "ran over");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("Base 10:               {}", 102);
    println!("Binary Base(2)       {:b}", 102);
    println!("Octal Base(8)        {:o}", 102);
    println!("Hexadecimal Base(16) {:x}", 102);

    // In formatting numbers can be aligned with spaces and other numbers
    println!("{number:<5}", number=1); // left-align or left-justify
    println!("{number:>5}", number=2); // right-align or right-justify

    println!("{number:0<6}", number=2); // output = 200000 mean width = 6, justified string digits = 0, actual number = 2
    println!("{number:1>7}", number=3); // output = 1111113 mean width = 7, justified string digits = 1, actual number = 3

    // Use named argument (width) by appending a `$`
    println!("{number:2>width$}", number=3, width=5); // output = 22223 mean width = 5, justified string digits = 2, actual number = 3

    // Rust checks the correct number of arguments are used
    // println!("My name is {1}, {0} {1}", "Bond"); // Incorrect
    println!("My name is {1}, {0} {1}", "James", "Bond"); // Correct

    // types that implement fmt::Display can be formatted with `{}`
    // User defined types cannot be formatted
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This won't {} print", Structure(3)); // This won't print coz Structure is user-defined & doesn't implement fmt::Display

    let number: f64 = 3.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Directly capture the arguments from the surrounding variables
    let number: f32 = 4.0;
    let width: usize = 7;
    println!("{number:1<width$}");

}