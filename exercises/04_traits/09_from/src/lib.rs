// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    pub value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> WrappingU32 {
        WrappingU32 { value: value }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
