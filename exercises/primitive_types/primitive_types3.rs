// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let a: [u32; 150] = [2; 150];
    // let mut array: [i32; 3] = [0; 3];
    
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        for x in a.iter() {
            println!("Value:\t{}", x);
        }
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
