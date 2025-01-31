// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// Floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
// The unit type (), whose only possible value is an empty tuple: ()

// Compound Types
//    Arrays like [1, 2, 3]
//    Tuples like (1, true)

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64; // this is the line it's inferred to be an i64

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21; // value changed

    // Error! The type of a variable can't be changed.
    // mutable = true; // errors because the mutable variable has a data type of i32, not data type bool

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array:[i32; 5] = [1, 2, 3, 4, 5]; // notice the semi colon in between the 5 and i32, if comma is used, an error occurs

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32); // you could also use suffix notation for the array. 

    // Note: notice the my_array requiring the definition of the slots for the array and the data type. This is because we didn't use suffix notation.
    // Note continued: use the suffix notation if many values are different data types
}
