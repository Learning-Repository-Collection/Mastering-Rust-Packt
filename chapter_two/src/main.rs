use crate::recursive::recurse;

mod looping;
mod while_loop;
mod recursive;

fn main() {
    println!("Hello, world!");

    looping::loop_through();
    looping::iterate();
    looping::iterate_vector();
    // looping::simple_loop();
    looping::while_loop();
    while_loop::while_looping();
    while_loop::break_loop();
    while_loop::loop_label();
    recursive::recurse(25);

}
