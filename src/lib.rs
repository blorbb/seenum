use std::num::NonZeroUsize;

pub use enum_select_derive::{Display, EnumSelect};

/// An enum trait for traversing through its variants.
///
/// This trait defines mappings from integer discriminants to variants,
/// as well as helper methods for traversing the variants, in a similar style
/// to the `checked_*`, `wrapping_*` and `saturating_*` operations on integers.
///
/// You should use the `#[derive(EnumSelect)]` macro to ensure that all
/// safety conditions are met. **Do not implement this yourself.**
///
/// # Safety
///
/// This trait must only be implemented on enums that:
/// - Are `#[repr(usize)]`
/// - Only contain unit variants (no tuple or named fields)
/// - Have at least one variant
/// - All variants have the default discriminant, so that discriminants
///   in the range `0..Self::COUNT` are all defined.
pub unsafe trait EnumSelect: Sized {
    /// The number of variants in the enum.
    const COUNT: NonZeroUsize;

    /// Converts an index discriminant to an enum variant. Does not perform
    /// any bounds checks.
    ///
    /// Use [`try_from_index`] instead for a safe version.
    ///
    /// [`try_from_index`]: EnumSelect::try_from_index
    ///
    /// # Safety
    /// The trait conditions must be met, as well as having `index` be in the
    /// the range `0..Self::COUNT` (not including `COUNT`).

    // This method can't have a default implementation as the size is unknown,
    // `std::mem::transmute` doesn't compile.
    unsafe fn from_index_unchecked(index: usize) -> Self;

    /// Converts an index discriminant to an enum variant.
    ///
    /// If the index is not within `0..Self::COUNT`, [`None`] is returned.
    fn try_from_index(index: usize) -> Option<Self> {
        if (0..Self::COUNT.into()).contains(&index) {
            // SAFETY: index is a valid discriminant
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

    /// Converts an enum to its index discriminant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// assert_eq!(Note::C.to_index(), 2);
    /// ```
    fn to_index(&self) -> usize {
        // https://doc.rust-lang.org/stable/reference/items/enumerations.html#pointer-casting
        // SAFETY: the enum has a #[repr(usize)]
        unsafe { *(self as *const Self as *const usize) }
    }

    /// Gets the first variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// assert_eq!(Note::first(), Note::A);
    /// ```
    fn first() -> Self {
        Self::try_from_index(0).expect("enum should have at least one variant")
    }

    /// Gets the last variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// assert_eq!(Note::last(), Note::G);
    /// ```
    fn last() -> Self {
        Self::try_from_index(usize::from(Self::COUNT) - 1)
            .expect("enum should have at least one variant")
    }

    /// Returns the next variant, wrapping back to the start if `self` is the
    /// last variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let f = Note::F;
    /// let g = Note::G;
    /// assert_eq!(f.wrapping_next(), Note::G);
    /// assert_eq!(g.wrapping_next(), Note::A);
    /// ```
    fn wrapping_next(&self) -> Self {
        Self::try_from_index((self.to_index() + 1) % usize::from(Self::COUNT))
            .expect("index should be within range 0..Self::COUNT")
    }

    /// Returns the previous variant, wrapping around to the last variant
    /// if `self` is the first variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let b = Note::B;
    /// let a = Note::A;
    /// assert_eq!(b.wrapping_prev(), Note::A);
    /// assert_eq!(a.wrapping_prev(), Note::G);
    /// ```
    fn wrapping_prev(&self) -> Self {
        Self::try_from_index(
            (self.to_index() + usize::from(Self::COUNT) - 1) % usize::from(Self::COUNT),
        )
        .expect("index should be within range 0..Self::COUNT")
    }

    /// Returns the next variant if there is one (`self` is not last).
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let f = Note::F;
    /// let g = Note::G;
    /// assert_eq!(f.checked_next(), Some(Note::G));
    /// assert_eq!(g.checked_next(), None);
    /// ```
    fn checked_next(&self) -> Option<Self> {
        if self.to_index() == usize::from(Self::COUNT) - 1 {
            None
        } else {
            Some(Self::try_from_index(self.to_index() + 1).expect("self should not be last"))
        }
    }

    /// Returns the previous variant if there is one (`self` is not first).
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let b = Note::B;
    /// let a = Note::A;
    /// assert_eq!(b.checked_prev(), Some(Note::A));
    /// assert_eq!(a.checked_prev(), None);
    /// ```
    fn checked_prev(&self) -> Option<Self> {
        if self.to_index() == 0 {
            None
        } else {
            Some(Self::try_from_index(self.to_index() - 1).expect("self should not be first"))
        }
    }

    /// Returns the next variant, saturating at the last variant if necessary
    /// (if `self` is last).
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let f = Note::F;
    /// let g = Note::G;
    /// assert_eq!(f.saturating_next(), Note::G);
    /// assert_eq!(g.saturating_next(), Note::G);
    /// ```
    fn saturating_next(&self) -> Self {
        self.checked_next().unwrap_or_else(Self::last)
    }

    /// Returns the previous variant, saturating at the first variant if
    /// necessary (if `self` is first).
    ///
    /// # Examples
    ///
    /// ```
    /// # use enum_select::EnumSelect;
    /// #[derive(Debug, PartialEq, Eq, EnumSelect)]
    /// #[repr(usize)]
    /// enum Note {
    ///     A, B, C, D, E, F, G
    /// }
    ///
    /// let b = Note::B;
    /// let a = Note::A;
    /// assert_eq!(b.saturating_prev(), Note::A);
    /// assert_eq!(a.saturating_prev(), Note::A);
    /// ```
    fn saturating_prev(&self) -> Self {
        self.checked_prev().unwrap_or_else(Self::first)
    }
}
