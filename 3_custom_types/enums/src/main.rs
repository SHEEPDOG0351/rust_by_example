// The enum keyword allows the creation of a type which may be one of a few different variants. 
// Any variant which is valid as a struct is also valid in an enum.

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
// enum WebEvent {
//     // An `enum` variant may either be `unit-like`,
//     PageLoad,
//     PageUnload,
//     // like tuple structs,
//     KeyPress(char),
//     Paste(String),
//     // or c-like structures.
//     Click { x: i64, y: i64 },
// }

// // A function which takes a `WebEvent` enum as an argument and
// // returns nothing.
// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         // Destructure `c` from inside the `enum` variant.
//         WebEvent::KeyPress(v) => println!("pressed '{}'.", v), // the 'v' variables can be whatever you want the name to be, think of them like function params
//         WebEvent::Paste(s) => println!("pasted \"{}\".", s), // same here 
//         // Destructure `Click` into `x` and `y`.
//         WebEvent::Click { x, y } => { // x and y represent the click's x and y properties, they count as standalone var's now (within this scope)
//             println!("clicked at x={}, y={}.", x, y); // they're used here 
//         },
//     }
// }

// fn main() {
//     let pressed = WebEvent::KeyPress('x');
//     // `to_owned()` creates an owned `String` from a string slice.
//     let pasted  = WebEvent::Paste("my text".to_owned());
//     let click   = WebEvent::Click { x: 20, y: 80 };
//     let load    = WebEvent::PageLoad; // just strings
//     let unload  = WebEvent::PageUnload; // just strings

//     inspect(pressed);
//     inspect(pasted);
//     inspect(click);
//     inspect(load);
//     inspect(unload);
// }
// ---------------------------------------------------------------- Type Aliases ---------------------------------------------------
// If you use a type alias, you can refer to each enum variant via its alias. 
// This might be useful if the enum's name is too long or too generic, and you want to rename it.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias. Now we can access the enum above from this name
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;


// The most common place you'll see this is in impl blocks using the Self alias.

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y, // realize how here the self keyword accesses the enum from above's properties
            Self::Subtract => x - y, // same here
        }
    }
}

fn main() {
    let x = Operations::Add; // as seen here
}

