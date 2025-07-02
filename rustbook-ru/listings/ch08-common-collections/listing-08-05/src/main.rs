fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let не_существует = &v[100];
    let не_существует = v.get(100);
    // ANCHOR_END: here
}
