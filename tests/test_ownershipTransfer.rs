/**
 * @test Test ownership transfer. 
 * 
 * If I pass a variable to the function or variable value to another variable,
 * 
 * ----------------------------------------------------------------------------
 * @result  if the value is transderred, then the original variable will not be able to access the value
 *          I have to explicitly clone the value to be able to access it
 */
fn main() {
    println!("This is a test for ownership transfer");

    let s1 = "hello".to_string();
    println!("s1: {}", s1);

    let s2 = s1;
    println!("s1: {}", s1); // This will throw an error because s1 has been moved to s2
    println!("s2: {}", s2);
}