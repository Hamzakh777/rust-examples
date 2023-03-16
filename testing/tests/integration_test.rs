// this is the library name
use testing;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(6, testing::add(2, 4));
}