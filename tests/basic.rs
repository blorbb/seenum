#![allow(dead_code)]

use enum_select::EnumSelect;

#[derive(Debug, PartialEq, EnumSelect)]
#[repr(usize)]
enum E {
    A,
    B,
    C,
    D,
}

#[test]
fn indices() {
    assert_eq!(usize::from(E::COUNT), 4);
    assert_eq!(E::A.to_index(), 0);
    assert_eq!(E::B.to_index(), 1);
    assert_eq!(E::C.to_index(), 2);
    assert_eq!(E::D.to_index(), 3);
    assert_eq!(E::try_from_index(2), Some(E::C));
    assert_eq!(E::try_from_index(4), None);
}

#[test]
fn traversal() {
    let first = E::first();
    assert_eq!(first, E::A);
    assert_eq!(first.checked_prev(), None);
    assert_eq!(first.wrapping_prev(), E::D);
    assert_eq!(first.saturating_prev(), E::A);
    assert_eq!(first.checked_next(), Some(E::B));
    assert_eq!(first.wrapping_next(), E::B);
    assert_eq!(first.saturating_next(), E::B);

    let last = E::last();
    assert_eq!(last, E::D);
    assert_eq!(last.checked_prev(), Some(E::C));
    assert_eq!(last.wrapping_prev(), E::C);
    assert_eq!(last.saturating_prev(), E::C);
    assert_eq!(last.checked_next(), None);
    assert_eq!(last.wrapping_next(), E::A);
    assert_eq!(last.saturating_next(), E::D);

    let third = E::try_from_index(2).unwrap();
    assert_eq!(third, E::C);
    assert_eq!(third.checked_prev(), Some(E::B));
    assert_eq!(third.wrapping_prev(), E::B);
    assert_eq!(third.saturating_prev(), E::B);
    assert_eq!(third.checked_next(), Some(E::D));
    assert_eq!(third.wrapping_next(), E::D);
    assert_eq!(third.saturating_next(), E::D);
}

fn slice() {
    let s = E::as_slice();
    assert_eq!(s, [E::A, E::B, E::C, E::D].as_slice());
}

fn main() {}
