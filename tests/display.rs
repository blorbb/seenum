#![allow(dead_code)]

use enum_select::{Display, EnumSelect};

#[derive(Debug, Display, EnumSelect)]
#[repr(usize)]
enum DurationType {
    #[display("1 minute")]
    Duration1m,
    #[display("5 minutes")]
    Duration5m,
    #[display("50 words")]
    Number50,
    #[display("100 words")]
    Number100,
    #[display("Endless")]
    Infinite,
}

#[test]
fn display() {
    let displays: Vec<_> = DurationType::as_slice()
        .iter()
        .map(ToString::to_string)
        .collect();

    assert_eq!(
        displays,
        ["1 minute", "5 minutes", "50 words", "100 words", "Endless"]
    );
}
