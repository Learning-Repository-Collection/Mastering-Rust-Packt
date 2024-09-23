
struct T {
    value: i32,
}

impl T {

    // Define a constructor
    fn new() -> T {
        T { value: 0 }
    }

}

pub(crate) fn main() {

    // Create a new instance of T
    let a = T::new();
    println!("Value in a: {}", a.value);

}