use polyglot_rs::*;

fn main() {
    println!("creating a new Java array");
    let array_type = java_type("int[]");
    let array = new_array_instance(array_type, 4);
    set_array_element(array, 2, 42);
    let element = as_i32(get_array_element(array, 2));
    assert_eq!(element, 42);
    assert_eq!(2, 2);
}
