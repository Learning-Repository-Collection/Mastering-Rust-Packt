
pub fn while_looping() {

    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9];
    let mut counter: usize = 0;
    let mut result = 0f32;
    let mut quit = false;

    while quit != true {
        if my_array[counter] > 1.5 {
            quit = true;
        } else {
            result += my_array[counter];
            counter += 1;
        }
    }

    println!("{}", result);

}

pub fn break_loop() {

    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9];
    let mut result = 0f32;

    for(_, value) in my_array.iter().enumerate() {
        if *value > 1.5 {
            break;
        } else {
            result += *value
        }
    }

    println!("{}", result);
}

pub fn loop_label() {

    'outer_loop: for x in 0..10 {
        'inner_loop: for y in 0..10 {
            if x % 2 == 0 { continue 'outer_loop; }
            if y % 2 == 0 { continue 'inner_loop; }
            println!("x: {}, y: {}", x, y);
        }
    }
}