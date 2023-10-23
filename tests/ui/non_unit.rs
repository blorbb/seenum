use seenum::EnumSelect;

#[derive(EnumSelect)]
#[repr(usize)]
pub enum Thing {
    None,
    Some(),
}

#[derive(EnumSelect)]
#[repr(usize)]
pub enum Thing2 {
    Some(i32),
}

#[derive(EnumSelect)]
#[repr(usize)]
pub enum Fields {
    Square { side_len: u32 },
    Dot,
}

fn main() {}
