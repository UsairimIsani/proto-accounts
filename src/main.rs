fn main() {
    let v = String::from("Chicken");
    let res = longest_with_an_announcement("chic", "mut", &v);
    println!("{}", res);
    let a: &'static str = "Chickem";
}
use std::fmt::{Debug, Display};
/// Start Here
/// # Example
/// ``` Chicken Chicken
///
///
/// ```
/// ### Mutton
pubfn longest_with_an_announcement<'a, 'b, 'c, T>(x: &'a str, y: &'b str, ann: &'c T) -> &'c T
where
    T: Debug,
{
    if x.len() > y.len() {
        ann
    } else {
        ann
    }
}
