pub fn primitive_types(){
    //signed integers
    let _signed_8_bit_integer: i8 = 1;
    let _signed_16_bit_integer: i16 = 1;
    let _signed_32_bit_integer: i32 = 1;
    let _signed_64_bit_integer: i64 = 1;
    let _signed_128_bit_integer: i128 = 1;
    let _signed_size_bit_integer: isize = 1;

    //unsigned integers
    let _unsigned_u8_bit_integer: u8 = 1;
    let _unsigned_u16_bit_integer: u16 = 1;
    let _unsigned_u32_bit_integer: u32 = 1;
    let _unsigned_u64_bit_integer: u64 = 1;
    let _unsigned_u128_bit_integer: u128 = 1;
    let _unsigned_usize_bit_integer: usize = 1;

    //floating point
    let _floating_point_32: f32 = 1.0;
    let _floating_point_64: f64 = 1.0;

    //char
    let _char: char = 'a';
    let _bool: bool = false;

    //compound types
    let _arrays: [i32; 5] = [1, 2, 3, 4, 5];
    let _tuple: (i32, bool) = (1, true);

    //literals can be used for integers, floats, chars, strings, booleans and unit types
    println!("1+2 = {}", 1i32 + 2);
    println!("1+2 = {}", 1i32 - 2);

    //short circuting boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("NOT true is {}", !true);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}