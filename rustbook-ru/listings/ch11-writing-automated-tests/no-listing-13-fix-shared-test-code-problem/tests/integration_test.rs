use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let итог = add_two(2);
    assert_eq!(result, 4);
}
