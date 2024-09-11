/**
 * Test the size of a pointer
 * ***************************
 * 
 * @Result:
 *    Size of a pointer is 8 bytes
 */
pub fn main( ) {
    use std::mem::size_of;

    // The size of a pointer is 8 bytes on a 64-bit system
    println!("Size of pointer is {} bytes", size_of::<usize>());
    println!("Size of u16 is {} bytes", size_of::<u16>());
    println!("Size of &u16 is {} bytes", size_of::<&u16>());
    // println!("Size of mut u16 {} bytes", size_of::<mut u16>()); // need to specify the lifetime
    println!("Size of &mut u16 {} bytes", size_of::<&mut u16>());
    println!("Size of a string is {} bytes", size_of::<String>());
    println!("Size of &String is {} bytes", size_of::<&String>());

}