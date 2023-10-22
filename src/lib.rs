use core::fmt;
use std::num::NonZeroUsize;

pub use enum_select_derive::EnumSelect;

pub trait EnumSelect: ConvertIndex {
    fn first() -> Self;
    fn last() -> Self;

    fn wrapping_next(&self) -> Self;
    fn wrapping_prev(&self) -> Self;
    fn checked_next(&self) -> Option<Self>
    where
        Self: Sized;
    fn checked_prev(&self) -> Option<Self>
    where
        Self: Sized;
    fn saturating_next(&self) -> Self;
    fn saturating_prev(&self) -> Self;
}

pub trait ConvertIndex {
    type Repr;
    const COUNT: NonZeroUsize;

    fn try_from_index(index: Self::Repr) -> Option<Self>
    where
        Self: Sized;
    unsafe fn from_index_unchecked(index: Self::Repr) -> Self;

    fn to_index(&self) -> Self::Repr;
}

// TODO: use unsafe?
macro_rules! impl_enum_select {
    ($repr:ty) => {
        impl<T> EnumSelect for T
        where
            T: ConvertIndex<Repr = $repr>,
        {
            fn first() -> Self {
                Self::try_from_index(0).expect("enum should have at least one variant")
            }

            fn last() -> Self {
                Self::try_from_index(expect_count_into::<$repr>(Self::COUNT) - 1)
                    .expect("enum should have at least one variant")
            }

            fn wrapping_next(&self) -> Self {
                Self::try_from_index((self.to_index() + 1) % expect_count_into::<$repr>(Self::COUNT))
                    .expect("index should be within range 0..Self::COUNT")
            }

            fn wrapping_prev(&self) -> Self {
                Self::try_from_index(
                    (self.to_index() + expect_count_into::<$repr>(Self::COUNT) - 1)
                        % expect_count_into::<$repr>(Self::COUNT),
                )
                .expect("index should be within range 0..Self::COUNT")
            }

            fn checked_next(&self) -> Option<Self>
            where
                Self: Sized,
            {
                if self.to_index() == expect_count_into::<$repr>(Self::COUNT) - 1 {
                    None
                } else {
                    Some(
                        Self::try_from_index(self.to_index() + 1).expect("self should not be last"),
                    )
                }
            }

            fn checked_prev(&self) -> Option<Self>
            where
                Self: Sized,
            {
                if self.to_index() == 0 {
                    None
                } else {
                    Some(
                        Self::try_from_index(self.to_index() - 1)
                            .expect("self should not be first"),
                    )
                }
            }

            fn saturating_next(&self) -> Self {
                self.checked_next().unwrap_or_else(Self::last)
            }

            fn saturating_prev(&self) -> Self {
                self.checked_prev().unwrap_or_else(Self::first)
            }
        }
    };
    ($($repr:ty)+) => {
        $(
            impl_enum_select!($repr);
        )+
    };
}

#[track_caller]
fn expect_count_into<T>(count: NonZeroUsize) -> T
where
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    T::try_from(usize::from(count)).expect("`Self::COUNT` cannot be greater than the repr size")
}
