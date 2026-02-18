use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
fn main() {
    let hello="Hello World";
    print_type_of(&hello);
    println!("Hello, world!");
}
