// It's possible to declare variable bindings first, and initialize them later. 
// However, this form is seldom used, as it may lead to the use of uninitialized variables.

fn main() {
    // Declare a variable binding
    let a_binding; // note how no value is given. This is what's called a variable binding. You could also give it a data type during this binding as well.

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x; // now the data type for the variable is i32
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
