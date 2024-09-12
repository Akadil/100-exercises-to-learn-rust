/**
 * Test the size of a slice
 * 
 * @Result:
 *      8 bytes for pointer
 *      8 bytes for metadata (length)
 */
fn main() {
    use std::mem::size_of;

    println!("🔎 Test the size of a slice");
    println!("");
    println!("Size of &str is {} bytes", size_of::<&str>());
    // println!("Size of str is {} bytes", size_of::<str>()); // won't compile
}