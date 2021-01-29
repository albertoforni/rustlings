// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

use std::ops::Range;

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in range(num) {
        println!("Ring! Call number {}", i + 1);
    }
}

fn range(i: i32) -> Range<i32> {
    return 0..i;
}
