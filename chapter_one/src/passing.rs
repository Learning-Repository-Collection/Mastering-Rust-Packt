
pub fn pass_values() {

    let add = add_values(3, 5);
    println!("{:?}", add);

    let mut var = 4;
    let ref_to_var = &var;
    let ref second_ref = var;
    let ref mut third_red = var;

}

pub fn add_values(a: i32, b: i32) -> i32 {
    a + b
}

pub fn by_reference() {

    let ref_num = &2;
    let deref_num = *ref_num;
    println!("ref_num is {}", deref_num);

}