
pub fn loop_through() {

    for x in 0..10 {
        println!("{}", x);
    }

    for x in (0..10).rev() {
        println!("{}", x);
    }

    for (i, j) in (10..20).enumerate() {
        println!("loop has been executed {} times. j = {}", i, j);
    }

}

pub fn iterate() {

    let my_array: [i32; 7] = [1i32, 3, 5, 7, 9, 11, 13];
    let mut value = 0i32;
    for(_, line) in my_array.iter().enumerate() {
        value += line
    }
    println!("{}", value);

}

pub fn iterate_vector() {

    let my_array = vec![1i32, 3, 5, 7, 9, 11, 13];
    let mut value = 0i32;

    for(_, line) in my_array.iter().enumerate() {
        value += line;
    }

    println!("{}", value);

}

pub fn simple_loop() {

    loop {
        println!("hello");
    }

}

pub fn while_loop() {

    let mut done = 0u32;
    while done != 32 {
        println!("done = {}", done);
        done += 1;
    }

}