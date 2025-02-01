// There are three types of structures ("structs") that can be created using the struct keyword:

// Tuple structs, which are, basically, named tuples.
// The classic C structs
// Unit structs, which are field-less, are useful for generics.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct Point { // A struct with two fields
    x: f32, // both being floats
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point, // both properties with Point type values. When you create a struct, you are basically making a new data type.
    bottom_right: Point, // the Point data type needing 2 values, the x property (which must be a float) and the y property (which must be a float).
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age }; // the field init shorthand syntax mentioned above

    // Print debug struct
    println!("{:?}", peter); // will error without ":?" trait as structs have the debug trait, not display trait

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 }; // aka an instance of point, creating a point instance. (For terminology sake)
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y); // x first, y after

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, y: 10.6 }; // did it

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point; // the variable won't be usable, but the properties made within would be, and they can be used separately, mking one of the few use cases of this destructuring

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair; // now the integer var represents the 1 made in pair's declaration, and decimal representing 0.1 for same reason

    println!("pair contains {:?} and {:?}", integer, decimal);

    fn rect_area(rectangle_struct: Rectangle) -> f32 {
        let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = rectangle_struct; // assign property values

        let width = (x2 - x1).abs(); // this line and the next 2 area needed for calculating purposes
        let height = (y1 - y2).abs(); // .abs() is used to ensure value is always positive. For instance, 9.5 - 10.5 would usually be -1, but here would be 1
        let area = (width * height * 100.0).round() / 100.0; // without the 100's the round function will try to round 1.1999999 whatever the hell and fail because the dec is too long, so we multiply by 100 to get 119.9, which then rounds to 120, then divide by the 100 again to get 1.2
        area // returns area
    }

    let test_rectangle = Rectangle {
        top_left: Point { x: 10.5, y: 10.9 },
        bottom_right: Point { x: 9.5, y: 9.7 }
    };

    fn square(square_struct: Point, width_height: f32) -> Rectangle {
        let Point { x: x1, y: y1 } = square_struct; // the var isn't available, but the x1 and y1 are, which will be equal to the values given in the square_struct param
        let bottom_measurement = ((y1 - width_height) * 100.0).round() / 100.0; // this is to deal with rounding
        Rectangle { top_left: Point{ x: x1, y: y1 }, bottom_right: Point { x: x1 + width_height, y: bottom_measurement}} // we return the rectangle instance, which we also define throughout this line.
        // the hardest part about these problems is just getting used to the syntax. Once you start to understand the syntax the problem becomes much easier
    }

    // println!("{:?}", rect_area(test_rectangle))
    let x = 10.6;
    let y = 11.2; 
    println!("{:#?}", square(Point{x, y}, 11.5))
}
