fn main() {
    println!("Hello, Zak!"); 

    println!("{} days", 31);

    println!("{:>5}", 1);


    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    //Format with precision
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}