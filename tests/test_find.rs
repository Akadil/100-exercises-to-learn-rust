use std::any::type_name;

/**
 * Check the nature of map(), find, get and blah blah 
 */
pub fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let result = v.iter().find(|&&x| x == 2);
    println!("{:?}", result); // Some(2)
    let result = v.iter().find(|&&x| x == 6);
    println!("{:?}", result); // None

    println!("Why the fuck I have 2 ampersand??");
    for elem in v.iter() {
        println!("Type of elem in loop is {}", type_of(&elem)); // i32
    }

}

/**
 * Function to know the type of something 
 */
fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}