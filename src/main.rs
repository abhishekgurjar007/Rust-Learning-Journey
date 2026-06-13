// Primitive data types:

// int, float, bool, char are primitive data types in Rust. They are the basic building blocks of data in Rust and are used to represent simple values.

// Integer:
// Rust has signed (+ & -) and unsigned (only +) integers. 
//i8, i16, i32, i64, i128, isize (signed integers)
//u8, u16, u32, u64, u128, usize (unsigned integers)

fn main(){
    let x: i32 = -42; // 32-bit signed integer
    let y: u64 = 100; // 64-bit unsigned integer

    println!("Signed integer 👌: {}", x);
    println!("Unsigned integer 👌: {}", y);


// Float:
// Rust has two primitive floating-point types: f32 and f64.
    
    let pi: f64 = 3.14159; // 64-bit floating-point number
    let e:  f32 = 2.71828; // 32-bit floating-point number

    println!("Float 👌:  {}", e);
    println!("Float 👌:  {}", pi);

// Boolean:
// The bool type represents a boolean value, which can be either true or false.
    let is_snowing: bool = true; // boolean value
    println!("Boolean 👌: {}", is_snowing);

// Character:
// The char type represents a single Unicode scalar value, which can be a letter, a number, a symbol, or even an emoji. It is denoted by single quotes ('').
    let letter: char = 'A'; // character
    let emoji: char = '😊'; // emoji

    println!("Character 👌: {}",letter);
    println!("Emoji 👌: {}", emoji);

}
