// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    // https://doc.rust-lang.org/rust-by-example/primitives/array.html
    let a = [42; 100];
    // https://stackoverflow.com/a/34684869/11895
    let a = vec![0; 100];
    let a = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    // for el in a.iter() {
    //     println!("{}", el);
    // }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
