use std::num::NonZeroUsize;

pub use enum_select_derive::EnumSelect;

/// This trait must only be implemented on unit enums with a `#[repr(usize)]`.
/// All variants must also have the default discriminant, so that discriminants
/// in the range `0..Self::COUNT` are all defined.
pub unsafe trait EnumSelect: Sized {
    const COUNT: NonZeroUsize;

    // This method can't have a default implementation as the size is unknown,
    // `std::mem::transmute` doesn't compile.
    unsafe fn from_index_unchecked(index: usize) -> Self;

    fn try_from_index(index: usize) -> Option<Self> {
        if (0..Self::COUNT.into()).contains(&index) {
            // SAFETY: index is a valid discriminant
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

    fn to_index(&self) -> usize {
        // https://doc.rust-lang.org/stable/reference/items/enumerations.html#pointer-casting
        // SAFETY: the enum has a #[repr(usize)]
        unsafe { *(self as *const Self as *const usize) }
    }

    fn first() -> Self {
        Self::try_from_index(0).expect("enum should have at least one variant")
    }

    fn last() -> Self {
        Self::try_from_index(usize::from(Self::COUNT) - 1)
            .expect("enum should have at least one variant")
    }

    fn wrapping_next(&self) -> Self {
        Self::try_from_index((self.to_index() + 1) % usize::from(Self::COUNT))
            .expect("index should be within range 0..Self::COUNT")
    }

    fn wrapping_prev(&self) -> Self {
        Self::try_from_index(
            (self.to_index() + usize::from(Self::COUNT) - 1) % usize::from(Self::COUNT),
        )
        .expect("index should be within range 0..Self::COUNT")
    }

    fn checked_next(&self) -> Option<Self> {
        if self.to_index() == usize::from(Self::COUNT) - 1 {
            None
        } else {
            Some(Self::try_from_index(self.to_index() + 1).expect("self should not be last"))
        }
    }

    fn checked_prev(&self) -> Option<Self> {
        if self.to_index() == 0 {
            None
        } else {
            Some(Self::try_from_index(self.to_index() - 1).expect("self should not be first"))
        }
    }

    fn saturating_next(&self) -> Self {
        self.checked_next().unwrap_or_else(Self::last)
    }

    fn saturating_prev(&self) -> Self {
        self.checked_prev().unwrap_or_else(Self::first)
    }
}
