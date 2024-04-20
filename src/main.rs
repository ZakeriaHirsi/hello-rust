use std::fmt;
#[derive(Debug)]
struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

fn main() {
    println!("Hello, Zak!");

    println!("{} days", 31);

    println!("{:>5}", 1);

    //{:?} is the same as {}
    println!("{:?} months in a year.", 12);
    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // println!("This struct {:?} will print...", Structure(3));
    // println!("This struct {:?} will print...", Deep(Structure(7)));

    //Format with precision
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("Test Print {:?}", Structure(5));
}


impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AAA{}", self.0)
    }
}
