/**
 * Test the nature of a lifetime of a reference
 * 
 * Results:
 *      1. The ownership and references are a big topic in Rust. We can create 
 *          several references and manipulate them. The compiler needs to keep
 *          track of what used where. 
 */
fn main() {
    let x = 10;
    // let r;
    {
        let y = 20;
        // r = dangerous_wParam(&x, &y);
        // r = dangerous_woParam(&x, &y);
    }
    // println!("r: {}", r);
}

fn dangerous_wParam<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

// fn dangerous_woParam(x: &i32, y: &i32) -> &i32 {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }
