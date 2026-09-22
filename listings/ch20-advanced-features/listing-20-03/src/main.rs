fn main() {
    // ANCHOR: here
    let mut число = 5;

    let r1 = &число as *const i32;
    let r2 = &mut число as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // ANCHOR_END: here
}
