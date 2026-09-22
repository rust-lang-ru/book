// ANCHOR: here
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let длина = значения.len();
    let ptr = значения.as_mut_ptr();

    assert!(mid <= длина);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.добавить(mid), len - mid),
        )
    }
}
// ANCHOR_END: here

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
