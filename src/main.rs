// use std;
// use std::fmt;
mod next_mod;
use next_mod::B as c;
mod odu;
pub mod zol {
    pub mod b {

        pub fn e() {}
    }
}
use zol::b;
fn main() {
    // // Using And Creating modules in rust
    c::some_in_b();
    b::e();
    odu::writing_in_mod();
    next_mod::do_something();
    // // Variables
    //
    // let a = 2;
    // println!("{}", a);
    // const a = 22;
    // let a = {
    //     let x = 1;
    //     x + 1
    // };
    // let x = y =8; does not work
    // println!("{}", a);
    //
    //
    // // mutable variables
    //
    // let mut a = 3; // use mut only with large data structures and use shadowing for small structures
    // println!("{}", a);
    // a = 5;
    // println!("{}", a);
    // a = a + 1;
    // println!("{}", a);
    //
    //
    // // Shadowing
    //
    // let a = 1;
    // println!("{}", a);
    // let a = a +1;
    // println!("{}", a);
    //
    //
    // // Data Types
    // // Scalar Types
    // // signed and unsigned
    // let a: u8 = 5; // unsigned 8bit, 16,32,64,128
    // println!("{}", a);
    // let a: usize = 5; // usize depends upon the type of architecture being targeted 32bit, 64bit, or incase of risc-v 128bit
    // println!("{}", a);
    // let a: i8 = 5; // signed 8,16,32,64,128-bit
    // println!("{}", a);
    // let a: isize = 5; // isize depends upon the type of architecture being targeted 32bit, 64bit, or incase of risc-v 128bit
    // // floats
    // let a = 5.0; //defaults to f64s
    // let a:f32  = 5.0 ;
    // println!("{}", a);
    // // char
    // let a = "C";
    // let cat_emoji = '😻' // emojis are valid utf-8 charachters
    //
    // // Compound Types
    // // Tuples are structs under the hood
    // let tup: (i32, i64) = (2, 45);
    // println!("{:?}", tup);
    // // Arrays are fixed width
    // let arr :[i32;2] = [23,23];
    // let arr = [5;5]; // would contain [5,5,5,5,5]
    //
    // // If  // there is no switch
    // if true{ // no brackets need other wise every thing same
    // println!("Chicken");
    // else{
    // println!("Kebab");
    // }
    // }
    // // if short
    // let is_of_age = if age >= 21 { true } else { false };
    //   println!("Is Of Age: {}", is_of_age)
    //
    // // Match (Incrredible)
    //
    // let mut n = 15;
    // match n {
    //     1 => println!("{}", n),
    //     2 | 3 | 5 | 7 => println!("Is Prime {}", n),
    //     15 => {
    //         println!("Chicken Is {} hundred Rupees LOng", n);
    //         n = n + 1;
    //         println!("Chicken Is {} hundred Rupees LOng", n);
    //     }
    //     11..=19 => println!("Is a teeeeen dabe wala{}", n),
    //     _ => println!("Nakara {}", n),
    // }
    //
    //
    // let pair = (56, 12);
    // let pair = (56, 0);
    // let pair = (0, 56);
    // match pair {
    //     (0, y) => println!("{} is the value of y", y),
    //     (x, 0) => println!("{} is the value of x", x),
    //     (x, y) => println!("The Value of X is {} and Y is {} ", x, y),
    //     _ => println!("HAHA Sucker aint going to work ever"),
    // }
    //
    //
    // let pair = (12, 12);
    // let pair = (56, 12);
    // let pair = (57, 12);
    // match pair {
    //     (x, y) if x == y => println!("X is equal to y ({}, {})", x, y), // guards in action
    //     (x, _) if x % 2 == 0 => println!("X is a even no. therefor {}", x),
    //     _ => println!("HAHAHA",),
    // }
    // let p = 56;
    // let n = match p {
    //     n @ 1..=20 => n,
    //     mut n @ 21..=56 => {
    //         n = n + 56;
    //         n
    //     }
    //     _ => 0,
    // };
    // println!("Value of n is {}", n);
    //
    //
    //
    // // loop
    //  loop{
    // println!("Something infinitely");
    // }
    // // returning from loop
    // let mut a = 0;
    // let counted = loop {
    //     if a == 10 {
    //         break a * 3;
    //     }
    //     a += 1;
    // };
    // println!("Counts {} , Counted {} ", a, counted);
    // // While
    // let mut a = 0;
    // while true {
    //     if a == 10 {
    //         break;
    //     } else {
    //         a += 1;
    //     }
    //     println!("Bun Kabab {}", a);
    // }
    // // for loop
    // let arr = [3; 8];
    // for something in arr.iter() {
    //     println!("Something in arr : {}", something);
    // }
    // // for with range
    // for i in 0..10 { // 0 inclusive 10 not inclusive
    // for i in 0..=10 { // 0 inclusive 10 also inclusive
    // println!("Something in arr : {}", i);
    // }
    // for i in (0..10).rev() {  // reverse range
    // println!("Something in arr : {}", i);
    // }
    //
    // // Struct
    // #[derive(Debug)]
    // struct Object {
    //     weight: u32,
    //     height: u32,
    // }
    // fn objector(obj: &Object) -> u32 {
    //     obj.height * obj.weight
    // }
    // let o = Object {
    //     height: 32,
    //     weight: 32,
    // };
    // let obj = Object::constructor(34, 56);
    // obj.debug();
    // println!(
    //     "{} {} {} {} {} {}",
    //     o.height,
    //     o.weight,
    //     objector(&o),
    //     o.area(),
    //     obj.area(),
    //     obj
    // );

    // impl Object {
    //     fn area(&self) -> u32 {
    //         &self.height * &self.weight
    //     }
    //     fn constructor(weight: u32, height: u32) -> Object {
    //         Object {
    //             weight: weight,
    //             height: height,
    //         }
    //     }
    //     fn debug(&self) {
    //         println!("{} {} {}", &self.height, &self.weight, &self.area());
    //     }
    // }
    // impl fmt::Display for Object {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         writeln!(f, "({} , {}) {}", &self.height, self.weight, &self.area())
    //     }
    // }
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // //Functions
    // some_unknown_function();
    // some_other_func_with_params(34.67, 56.56);
    // let a = func_with_return(3);
    // println!("{}", a)
    // // // Ownership
    // let a = 3;
    // let b = String::from("Chicken"); // value is dropped bcz string is allocated therefore it is moved.
    //                                  // take_something(a, b);
    //                                  // println!("Print x {}, Print y  {} : ", a, b);
    // let a = takes_and_gives_back(b);
    // println!("a  : {} ", a);
    //
    // // References and Borrowing
    // // A varibale when refrenced it borrow the value from the owner and return it back
    // // The reference cannot be mutated unless specified.
    // // But mutable references have one big restriction:
    // // you can have only one mutable reference to a particular piece of data in a particular scope.
    // let a = String::from("Some Text`");
    // let b = give_ref(&a);
    // println!("{}", b);
    // let mut some_string = String::from("Some String getting Attached to ");
    // let (b, val) = change_ref(&mut some_string);
    // println!("{} , {}", b, val);
    // let mut some_val = &mut some_string;
    // let (a, val_1) = change_ref(&mut some_val);
    // println!("{} , {}", a, val_1);
}

// //Functions
// fn some_unknown_function() {
//     println!("Chicken");
// }{
// fn some_other_func_with_params(x: f32, y: f32) {
//     println!("Chicken itna {}, kabab itna {}", x, y);
// }
// fn func_with_return(x: i32) -> i32 {
//     x + 1
// }
// fn take_something(x: i32, y: String) {
//     println!("Print x : {}, Print y : {}  ", x, y);
// }
// fn gives_ownership() -> String {
//     // gives_ownership will move its
//     // return value into the function
//     // that calls it
//     let some_string = String::from("hello"); // some_string comes into scope
//     some_string // some_string is returned and
//                 // moves out to the calling
//                 // function
// }
// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string // a_string is returned and moves out to the calling function
// }
// fn give_ref(string: &String) -> usize {
//     let a = string.len();
// return a; // both work
// a         // both work
// }
// fn change_ref(stri: &mut String) -> (usize, &mut String) {
//     stri.push_str("Chicken");
//     (stri.len(), stri)
// }
