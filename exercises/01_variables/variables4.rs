// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // mutability to change x later 
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
