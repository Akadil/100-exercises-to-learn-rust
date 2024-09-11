/**
 * Test the nature of ampersand in front of a variable and in front of a type. 
 * Are they different?
 * ****************************************************************************
 * @version 
 *          silly, beginner test
 * 
 * @Results:
 *      1. Ampersand is the same everywhere. The doc said that & in front of 
 *         a type and a variable are the different. Yes they are different but
 *         only for us, internally they are the same concept.
 * 
 *      2. when you give a value for a newly create reference variable, you have 
 *         to assign it with an ampersand otherwise it won't allow
 */
fn main() {
    
    println!("Test simple version with assigning to an ampersand variable");
    let a: u32 = 10;
    let b: &u32 = &a;

    println!("a: {}", a);
    println!("b: {}", b);

    println!("Test simple version with assigning to not an ampersand variable");
    // let c: &u32 = a;     // Compiler won't allow this

    println!("a: {}", a);
    // println!("c: {}", c);
}