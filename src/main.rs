use std::fmt;
#[derive(Debug)]
enum Family {
    Father {
        name: String,
        height: u8,
        age: u8,
        occupation: String,
    },
    // Mother {
    //     name: String,
    //     height: u8,
    //     age: u8,
    //     occupation: String,
    // },
    // Son {
    //     name: String,
    //     height: u8,
    //     age: u8,
    // },
    // Daughter {
    //     name: String,
    //     height: u8,
    //     age: u8,
    // },
}
#[derive(Debug)]
struct Father {
    name: String,
    height: u8,
    age: u8,
    occupation: String,
}
impl Father {
    fn constructor(name: String, height: u8, age: u8, occupation: String) -> Father {
        Father {
            name,
            height,
            age,
            occupation,
        }
    }
    fn introduce(&self) {
        println!("{} ", &self)
    }
}
impl fmt::Display for Father {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Hi, I am {}, I am  {} old, I am {} tall and I am a {}",
            &self.name, &self.age, &self.height, &self.occupation
        )
    }
}
fn main() {
    let father = Father::constructor(
        String::from("Altaf Hussain"),
        128,
        55,
        String::from("Doctor"),
    );
    father.introduce();
}
