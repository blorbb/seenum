#![allow(dead_code)]

use enum_select::Display;

#[derive(Display)]
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
