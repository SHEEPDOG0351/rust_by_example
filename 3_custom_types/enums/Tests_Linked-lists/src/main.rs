// A common way to implement a linked-list is via enums:

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node. (A node holds it's elements value and points to the next)
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil, // Nil can be used to end a list as it has no value itself, (and assuming it's used at the end in declaration of the list),
    // then the Rust compiler when assigning the pointers to the lsit won't assign one to Nil, as it has no value or value after it either.
    // In case your curious, if Nil was placed in the middle of the list, it would error as it doesn't match u32.
    // Nil can be used at the end of the list since it doesn't require itself to be a u32 (as it's not defined here within it's declaration):
    // let list = List::Cons(1, Box::new(
    //     List::Cons(2, Box::new(
    //         List::Cons(3, Box::new(
    //             List::Nil // ✅ Allowed, as there's no u32 required from the above declaration
    //         ))
    //     ))
    // ));
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has / holds type `List`, so when the new function is called, an empty list is returned as it holds the type list, but no values.
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self)) // the second part of this function call tells the prepend function to add the rest of the old list's values on to the end of the list,
        // This is why the new list made doesn't lose the values of the old list during the consumption of ownership. 
        // Elem is the element that the user wants to put at the front of the list, which gets put at the front as it's the first argument to the Cons property.
    }

    // Return the length of the list
    fn len(&self) -> u32 { // reference required here as we don't want to consume the list, thus making it unusable after the functions ran on it. Also, we don't want to change the value of the original list to it's length.
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self { // '*' is required as you need to access the List enum's underlying type for that element within the generated list (that being either Cons or Nil)
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(), // here, the recursion will keep continuing until a Nil is reached. This is because the part 'tail.len()' is actually calling the length of the next element in the list, (not the currently iterated one). Then it checks to see if it's a Cons type, if not, then it stops as the .len() function isn't called on Nil (as we don't tell it to within the code)
            // But yea realize how the above line is recursion , didn't realize that for like 15 mins because I'm retarded.
            // Lastly, the '_' represents the numerical value of the node, which we don't use and thus is good practice to make an _
            // Base Case: An empty list has zero length
            Nil => 0 // you need someting to stop the recursion, that's what Nil is used here for (as mentioned above with base case).
            // Since the Nil type will always be after the last value of the list, we can tell the recursion to stop using this as a stop sign (base case).
            // It does so by not calling the .len() function, thus never iterating over another value, (like it does above, thus keeping the recursion going when the type value within the List's currently iterated over element is Cons)
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String { // we want to return a new heap allocated version of the string given, thus don't want ownership of the string as we would consume it, thus making it unsuable after this functions call
        match *self { // we want to access the type of the underlying value, as we want to see if it's Cons or Nil, thus the '*' is required
            Cons(head, ref tail) => { // head here represents the numerical value behind the node, tail representing the type of the value (that being either Cons or Nil)
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify()) // just like the previous recursive call, the .stringify() here calls on the next element, thus keeping the recursion going.
            },
            Nil => {
                format!("Nil") // here, .stringify() isn't called, thus ending the recursion as it's not being told to continue on to the next element within the list.
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1); // adds 1 to the front of the list made above
    list = list.prepend(2); // then 2
    list = list.prepend(3); // then 3. (Realize how since 3 is added last, it's at the front of the list as that's how the prepend function's designed)

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
