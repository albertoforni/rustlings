// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

// The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

const NUMBER: u16 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
