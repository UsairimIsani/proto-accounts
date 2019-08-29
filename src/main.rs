// Enums
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// impl IpAddr {
//     fn constructor(t: (u8, u8, u8, u8)) -> IpAddr {
//         IpAddr::V4(t.0, t.1, t.2, t.3)
//     }
// }
// fn main() {
//     // let home = IpAddr::V4(127, 0, 0, 1);
//     let home = IpAddr::constructor((34, 44, 4, 4));
//     println!("{:?}", &home);

//     let loopback = IpAddr::V6(String::from("::1"));
// }
// use std::fmt;
// #[derive(Debug)]
// enum Family {
//     Father {
//         name: String,
//         age: u8,
//         occupation: String,
//     },
// }
// impl Family {
//     fn constructor(father: (String, u8, String)) -> Family {
//         Family::Father {
//             name: father.0,
//             age: father.1,
//             occupation: father.2,
//         }
//     }
// }
// fn main() {
//     let father = Family::constructor((String::from("Altaf"), 55, String::from("Doctor")));
//     println!("{:?}", &father);
// }
// fn main() {
//     let a = Some("Stringy thing");
//     let b = Some(45);
//     let c: Option<i32> = None;
//     println!("{:?} {:?} {:?}", a, b, c);
//     let d: u8 = match b {
//         Some(i) => i as u8,
//         None => 0,
//     };
//     let e: u8 = match a {
//         Some(i) => i.len() as u8,
//         None => 0,
//     };
//     println!("{}", d);
//     println!("{}", e);
// }
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn cons(name: String, age: u8) -> Option<Person> {
        Some(Person { name, age })
    }
}
fn main() {
    let p = Person::cons(String::from("Usairim Isani"), 21);
    // let p: Option<Person> = None;
    println!("{:?} ", &p);
    let mut p = match p {
        Some(i) => i,
        _ => panic!("all are Chickens, all Person fled"),
    };
    // if let Some(Person) = p {
    //     p
    // };
    let b = Person::cons(String::from("name: String"), 32);
    // let mut c;
    if let Some(i) = &b {
        println!("{:?}", &b);
        println!("{:?}", &i);
        // c = b.unwrap();
    };
    println!("{:?}", &b);
    println!("{:?}", &p);
    // println!("{:?}", &c);
}
