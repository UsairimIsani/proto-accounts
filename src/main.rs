#[derive(PartialEq, Debug)]
struct Shirt {
    size: u32,
    brand: String,
    quality: u32,
    quantity: u32,
}
impl Shirt {
    fn new(size: u32, brand: String, quality: u32, quantity: u32) -> Shirt {
        Shirt {
            size,
            brand,
            quality,
            quantity,
        }
    }
}
impl Iterator for Shirt {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.quantity >= 0 {
            self.quantity -= 1;
            Some(self.quantity)
        } else {
            println!("{}", self.quantity);
            None
        }
    }
}
fn main() {
    let mut a: Shirt = Shirt::new(15, String::from("Nike"), 50, 70);
    println!("{:?}", a);
    let b = a.quantity;
    let b = b as i32;
    for i in 0..b {
        let some = &a.next();
        match some {
            Some(t) => println!("Shirts Here {:?}", t),
            None => println!("All Out"),
        }
    }
    println!("{:?}", a);
}
