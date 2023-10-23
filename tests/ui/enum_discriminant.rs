use seenum::EnumSelect;

#[derive(EnumSelect)]
#[repr(usize)]
pub enum Something {
    Abc = 0,
    Xyz = 2,
}

fn main() {}
