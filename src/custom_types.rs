// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//Single unit
struct Unit;

//tuple
struct Pair(i32, f32);

//Struct with 2 fields
struct Point {
    x: f32,
    y: f32,
}

//Struct that can be reused from other structs
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn _custom_types() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27; //DAMN
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 1.0, y: 1.0 };
    let another_point: Point = Point { x: 1.0, y: 1.0 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
}

pub fn _rect_area() {
    let _rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 4.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };

    let length: f32 = _rectangle.bottom_right.x - _rectangle.top_left.x;
    let height: f32 = _rectangle.top_left.y - _rectangle.bottom_right.y;

    let area: f32 = length * height;

    println!("{:?}", area);
}

pub fn _inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn _WebEvent(){
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    _inspect(pressed);
    _inspect(pasted);
    _inspect(click);
    _inspect(load);
    _inspect(unload);
}
