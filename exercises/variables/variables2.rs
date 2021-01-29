// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x = 5;
    print_type_of(&x);
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
