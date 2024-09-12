/**
 * Test the conversion from one type to another (numbers only)
 * 
 * @Result:
 *      
 */
fn main() {
    println!("Test conversion from i32 to i64 with 'as' keyword");
    {    
        let a: i32 = 42;
        let b: i64 = a as i64;
        println!("{} -> {}", a, b);
    }
    println!("Test conversion from i32 to i64 without 'as' keyword");
    // {
    //     let a: i32 = 42;
    //     let c: i64 = a;

    //     println!("{} -> {}", a, c);
    // }
}
