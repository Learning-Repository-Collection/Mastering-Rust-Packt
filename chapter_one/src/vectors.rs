

pub fn vector_call() {

    let mut my_vector: Vec<f32> = Vec::new(); // explicit definition
    let mut my_alt_vector = vec![4f32, 3.14, 6.28, 13.54, 27.08];

    let mut my_ids: Vec<i64> = Vec::with_capacity(30);
    let my_vec: Vec<u64> = (0..10).collect();

    println!("{:?}", my_vector);
    println!("{:?}", my_alt_vector);
    println!("{:?}", my_ids);
    println!("{:?}", my_vec);

    let mut new_vec : Vec<i32> = (0..10).collect();
    println!("{:?}", new_vec);
    new_vec.push(13);
    new_vec.push(21);
    println!("{:?}", new_vec);
    let mut twenty_one = new_vec.pop();
    println!("twenty_one={:?}", twenty_one);
    println!("{:?}", new_vec);

    let my_slice = &my_vec[1..5];
    println!("{:?}", my_slice);

}