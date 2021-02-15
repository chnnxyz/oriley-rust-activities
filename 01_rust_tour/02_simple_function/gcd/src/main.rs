// mut stands for mutable variable, &mut stands for mutable pointer. u64 is unsigned 64 bit integer
// -> will define the return type (instead of declaring as prefix like in C++)
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

fn main() {

}

// #[test] is an attribute to initialize test cases
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}