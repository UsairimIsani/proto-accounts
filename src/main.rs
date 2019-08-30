use std::collections::HashMap;
fn main() {
    life_time_playground()
}
fn life_time_playground() {
    let text = "hello world world world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{} ", count);
    }
}
