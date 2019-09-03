use std::fmt::Debug;
enum Shirt<T>
where
    T: Debug + PartialEq,
{
    Co(T),
    Cd,
}
use crate::Shirt::{Cd, Co};
fn main() {
    let chic = Co(Box::new([1, 2, 4, 5, 6]));
    let some: Result<_, _> = match chic {
        Co(s) => Ok(s),
        Cd => Err(0),
    };
    println!("{:?}", some.unwrap());
}
