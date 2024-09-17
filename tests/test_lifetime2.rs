/**
 * Let's try to break the lifetime
 * 
 * @Results:
 *      1. It's interesting that lifetime and ownership principles eliminates
 *          penetratable points. The rules are made to prevent the dangling
 */
fn main() {
    let holder = 10;

    let borrow1 = &holder;
    let borrow2 = &holder;

    function(borrow1, borrow2);
}

fn function(borrow1: &i32, borrow2: &i32) {
    println!("borrow1: {}, borrow2: {}", borrow1, borrow2);

    
}

