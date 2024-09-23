
mod strings;
mod arrays;
mod type_name;
mod vectors;

/// a simple function example
fn main() {

    println!("Hello, world!");
    println!("Tests have not been compiled, use rustc --test instead (or cargo test)");

    strings::string_module();
    strings::build_string();
    type_name::main();
    arrays::make_array();


}

#[test]
fn multiply_test() {
    if 2 * 3 == 5 {
        println!("The multiply worked");
    }
}

#[test]
fn multiply() {
    assert_eq!(6, 2 * 3)
}