use std::ffi::{c_char, CString};

use polyglot_sys::{polyglot_as_i32, polyglot_value};

#[no_mangle]
pub fn java_type(classname: &str) -> polyglot_value {
    let classname = CString::new(classname).unwrap();
    let classname: *const c_char = classname.as_ptr() as *const c_char;
    unsafe { polyglot_sys::polyglot_java_type(classname) }
}

#[no_mangle]
pub fn new_array_instance(array_object: polyglot_value, len: i32) -> polyglot_value {
    unsafe { polyglot_sys::polyglot_new_instance(array_object, len) }
}

#[no_mangle]
pub fn set_array_element(array: polyglot_value, idx: i32, value: i32) {
    unsafe { polyglot_sys::polyglot_set_array_element(array, idx, value) };
}

#[no_mangle]
pub fn get_array_element(array: polyglot_value, idx: i32) -> polyglot_value {
    unsafe { polyglot_sys::polyglot_get_array_element(array, idx) }
}

#[no_mangle]
pub fn as_i32(value: polyglot_value) -> i32 {
    unsafe { polyglot_as_i32(value) }
}

#[cfg(test)]
mod test {
    use super::*;

    // Example from https://www.graalvm.org/22.2/reference-manual/llvm/Interoperability/#polyglot-c-api
    #[test]
    fn array_test() {
        let array_type = java_type("int[]");
        let array = new_array_instance(array_type, 4);
        set_array_element(array, 2, 42);
        let element = as_i32(get_array_element(array, 2));
        assert_eq!(element, 42);
        assert_eq!(2, 2);
    }
}
