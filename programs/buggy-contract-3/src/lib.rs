fn main() {
    let ptr: *mut i32 = std::ptr::null_mut();

    unsafe {
        // Dereferencing a null pointer
        println!("Dereferenced value: {}", *ptr);
    }
}
