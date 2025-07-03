extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute значение of -3 according to C: {}", abs(-3));
    }
}
