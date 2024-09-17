/**
 * Test the interesting struct construction
 * 
 * Results:
 *      The struct
 *          struct Owner2(i32, i16);
 *      creates tuple struct
 */
fn main() {
    let mut owner = Owner(18);
    let mut owner2 = Owner2(18, 20);

    println!("Test the first owner");
    owner.add_one();
    owner.print();
    println!();

    println!("Test the second owner");
    owner2.add_one();
    owner2.print();
}

struct Owner(i32);

#[derive(Debug)]
struct Owner2(i32, i16);

impl Owner {
    fn add_one<'a>(&'a mut self) { 
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

impl Owner2 {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
        println!("`print`: {}", self.1);
        println!("`print`: {:?}", self);
    }
}