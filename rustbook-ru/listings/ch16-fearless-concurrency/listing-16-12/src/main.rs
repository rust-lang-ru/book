use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut число = m.lock().unwrap();
        *число = 6;
    }

    println!("m = {m:?}");
}
