fn st(s: &str) -> String {
    String::from(s)
}
fn main() {
    unsafe {
        let mut vec: Vec<i32> = vec![2, 3, -65, 90];
        let size = vec.capacity() as isize - 1;
        let ptr_to_vec = vec.as_mut_ptr();
        let end_addr = ptr_to_vec.offset(size);
        println!(
            "Vector : {:?}, \nSize : {},\nPointer : {:?}\nEnd of Address : {:?}",
            vec, size, ptr_to_vec, &end_addr
        );
        for i in 0..=size {
            let j = i.clone() as isize;
            println!(
                "Got the Pointer {:?} and its value {:?}",
                ptr_to_vec.offset(j),
                *ptr_to_vec.offset(j)
            );
        }
        // Vector : [2, 3, -65, 90],
        // Size : 4,
        // Pointer : 0x55e20f377b40
        // End of Address : 0x55e20f377b50
        // Got the Pointer 0x55e20f377b40 and its value 2
        // Got the Pointer 0x55e20f377b44 and its value 3
        // Got the Pointer 0x55e20f377b48 and its value -65
        // Got the Pointer 0x55e20f377b4c and its value 90
        // Got the Pointer 0x55a8f6342b50 and its value 0
    }
    // while_let();
}
fn while_let() {
    let mut a: Vec<u8> = vec![123, 123, 123, 123, 123];
    while let Some(i) = a.pop() {
        print!("{}  ", i);
        println!("{:?}", &a);
    }
}
