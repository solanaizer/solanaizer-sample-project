use std::ptr;

fn main() {
    let mut data: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Creating a raw pointer to the first element of the array
    let ptr_to_first_element = &mut data[0] as *mut i32;

    unsafe {
        *ptr::null_mut() = 10; 
    println!("Modified value: {}", data[0]); 
}
