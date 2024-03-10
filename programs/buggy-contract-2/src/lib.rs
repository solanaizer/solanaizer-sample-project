use std::ptr;

fn main() {
    let mut buffer: [u8; 5] = [0; 5];

    // Create a pointer to the buffer
    let mut ptr = buffer.as_mut_ptr();

    // Iterate over the buffer and write values
    for i in 0..6 {
        // Dereference the pointer and assign a value
        unsafe {
            ptr::write(ptr, i as u8);
        }

        // Increment the pointer to the next element
        ptr = ptr.offset(1);
    }

    // Print the buffer
    println!("{:?}", buffer);
}
