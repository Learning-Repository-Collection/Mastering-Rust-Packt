

pub fn make_array() {

    let my_array = ["Merseybus", "Amberline", "Crosville", "Liverbus", "Liverline", "Fareway"];
    println!("{:?}", my_array);

    let mut my_array_two: [i32; 4] = [1, 11, 111, 1111];
    let mut empty_array: [&str; 0] = [];
    let number = [111; 5];

    println!("{:?}{:?}{:?}", my_array_two, empty_array, number)

}

