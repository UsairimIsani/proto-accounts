fn main() {
    let a = 55;
    let b = &a;
    let c = &b;
    assert_ne!(55, **c);
}
