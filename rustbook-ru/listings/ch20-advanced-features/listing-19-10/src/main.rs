static mut СЧЁТЧИК: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        СЧЁТЧИК += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("СЧЁТЧИК: {СЧЁТЧИК}");
    }
}
