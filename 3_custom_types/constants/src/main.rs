// Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

// const: An unchangeable value (the common case).
// static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. 
// Accessing or modifying a mutable static variable is unsafe.

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust"; // still work, is also immutable
const THRESHOLD: i32 = 10; // still work, const makes it immutable as well
// let s: &'static str = "hello world"; // as you can see, this line still errors as s isn't defined yet, so we're referencing nothing.
// The point of this line though is to show you that you could reference a value from a variable that would otherwise be locally scoped and make it global with static.

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD // returns a boolean, if n is bigger true, else false
}

fn main() {
    let n = 16;
    LANGUAGE = "Rust2";
    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}