// use is the equivalent to import, also, :: is equivalent to py's .
// Write and FromStr are called traits (essentially classes)
// we use write since std::io:stderr and writeln! implement
// Write or a method write_fmt
// ur4::from_str calls std::str::FromStr 
use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    //assert statement to validate both integers are greater than 0
    //semicolons matter, and logical operators are like in java
    //we asserting with excl sign now i guess. Ok excl sign is macro instead of function
    //macros are run at compile time. Assertions cannot be skipped unless they are debug_assert!
    assert!(n != 0 && m != 0);
    //Good practice not to use parentheses on control structures I guess?
    while m != 0 {
        if m < n {
            // let is for declaring variables, ES6 much?
            // let is kind of dynamic typing i guess
            // can declare and instantiate in one move.
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // statements with no semicolon at the end of a function are returns()
    n
}

// in void functions we omit the arrow
fn main() {
    let mut numbers = Vec::new();
    //creates a new mutable instance of a vector called numbers
    //the mut part lets us change the structure
    //Vec<u64> would be the strict declaration
    for arg in std::env::args().skip(1){
    // std::env::args() returns an iterator, it's first value is the name
    // of the program being run, so we skip it
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        //expect prints when finding an exception
        //u64::fromstr returns a result Ok(v) or Err(e)
        //Errors are handled by using results or panic, expext parses it
        //Whenever using a Result, use expect
    }

    if numbers.len() == 0 {
        // No GCD for a  empty set of numbers
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        // unwrap() loooks for errors when printing the error message
        // can also use expect
        std::process::exit(1);
        //exit with code 1
    }

    let mut d = numbers[0];
    // d needs to be mutable to assign to the loop
    for m in &numbers[1..] {
        // & is pointer * is pointer value
        // or reference and dereference
        d = gcd(d, *m);
        // d is updated to be the gcd of the previous two elements and the new one
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d)


    // rust assumes if main doesnt fail it exits with code 0
}
