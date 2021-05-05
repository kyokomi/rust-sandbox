extern crate a5b356bf40b6228103daadbd0c102f4af as adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
