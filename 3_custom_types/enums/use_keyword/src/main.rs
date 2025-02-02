// The use declaration can be used so manual scoping isn't needed:

// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// enum Stage {
//     Beginner,
//     Advanced,
// }

// enum Role {
//     Student,
//     Teacher,
// }

// fn main() {
//     // Explicitly `use` each name so they are available without
//     // manual scoping.
//     use crate::Stage::{Beginner, Advanced}; // below you'll see that these can now be accessed as if they're variables (direct values)
//     // Automatically `use` each name inside `Role`.
//     use crate::Role::*;

//     // Equivalent to `Stage::Beginner`.
//     let stage = Beginner; // these are the lines I was talking about in the comment above at line 19
//     // Equivalent to `Role::Student`.
//     let role = Student; // these are the lines I was talking about in the comment above at line 19

//     match stage {
//         // Note the lack of scoping because of the explicit `use` above.
//         Beginner => println!("Beginners are starting their learning journey!"),
//         Advanced => println!("Advanced learners are mastering their subjects..."),
//     }

//     match role {
//         // Note again the lack of scoping.
//         Student => println!("Students are acquiring knowledge!"),
//         Teacher => println!("Teachers are spreading knowledge!"),
//     }
// }

 // ---------------------------------------------------------------- C-Like ---------------------------------------------------------

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero, // by default, equal to 0 as it's the first property
    One, // then 1
    Two, // then 2
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
