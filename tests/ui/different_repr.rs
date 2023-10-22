use enum_select::EnumSelect;

#[derive(EnumSelect)]
#[repr(u32)]
pub enum Something {
    Abc,
    Xyz,
}

fn main() {}
