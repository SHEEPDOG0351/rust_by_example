// Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.

fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
        // end of block, thus end of scope, any variables declared within this block will be deleted from memory after this block runs, thus unusable down the line
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc"; // Note: shadowing isn't tied to scopes. Meaning that this still works outside the scope,
        // though behavior slightly differs as the original value of the variable (and possibly the type) gets completely re-written,
        // where here, the variable returns back to its original value once the scope ends.

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding); // notice how the shadowed_binding variable is back to 1 now as the previous scope ended

    // This binding *shadows* the previous binding
    let shadowed_binding = 2; // since we re-initialize the variable with 'let', it allows us now to change the value even know it was immutable originally
    // This is allowed as its considered shadowing a variable. Since this was done to the main scope, the original value will be replaced and thus won't be returned to automatically (meaning you have to define that behavior)
    println!("shadowed in outer block: {}", shadowed_binding);
}
