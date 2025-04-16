use std::{num::NonZeroUsize, ops::Index};
use then::then;

#[derive(Debug, Clone, Copy)]
pub struct NonEmptySlice<'a, T> {
    slice: &'a [T],
}

impl<'a, T> NonEmptySlice<'a, T> {
    /// Returns a new `NonEmptySlice` if `slice` is not empty, otherwise `None`.
    pub const fn new(slice: &'a [T]) -> Option<Self> {
        then!(!slice.is_empty(), Self { slice })
    }

    /// Returns a new `NonEmptySlice`.
    /// # Safety
    /// `slice` must be known to not be empty or this could lead to undefined behavior later.
    pub const unsafe fn new_unchecked(slice: &'a [T]) -> Self {
        Self { slice }
    }

    /// Returns a reference to the head of the slice.
    #[must_use]
    pub const fn head(self) -> &'a T {
        unsafe { self.slice.split_first().unwrap_unchecked().0 }
    }
    /// Returns a reference to the tail of the slice.
    #[must_use]
    pub const fn tail(self) -> &'a [T] {
        unsafe { self.slice.split_first().unwrap_unchecked().1 }
    }

    /// Returns the length of the slice.
    #[must_use]
    pub const fn len(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.slice.len()) }
    }

    /// Returns whether the slice is empty.
    ///
    /// This will always return `false` since this type by its nature cannot be empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        false
    }
}

impl<T> Index<usize> for NonEmptySlice<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.slice.index(index)
    }
}

#[derive(Debug)]
pub struct NonEmptyMutSlice<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T> NonEmptyMutSlice<'a, T> {
    /// Returns a new `NonEmptyMutSlice` if `slice` is not empty, otherwise `None`.
    pub const fn new(slice: &'a mut [T]) -> Option<Self> {
        then!(!slice.is_empty(), Self { slice })
    }

    /// Returns a new `NonEmptyMutSlice`.
    /// # Safety
    /// `slice` must be known to not be empty or this could lead to undefined behavior later.
    pub const unsafe fn new_unchecked(slice: &'a mut [T]) -> Self {
        Self { slice }
    }

    /// Returns a reference to the head of the slice.
    #[must_use]
    pub const fn head(self) -> &'a T {
        unsafe { self.slice.split_first().unwrap_unchecked().0 }
    }

    /// Returns a reference to the head of the slice.
    #[must_use]
    pub const fn head_mut(self) -> &'a mut T {
        unsafe { self.slice.split_first_mut().unwrap_unchecked().0 }
    }

    /// Returns a reference to the tail of the slice.
    #[must_use]
    pub const fn tail(self) -> &'a [T] {
        unsafe { self.slice.split_first().unwrap_unchecked().1 }
    }

    /// Returns a reference to the tail of the slice.
    #[must_use]
    pub const fn tail_mut(self) -> &'a mut [T] {
        unsafe { self.slice.split_first_mut().unwrap_unchecked().1 }
    }

    /// Returns the length of the slice.
    #[must_use]
    pub const fn len(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.slice.len()) }
    }

    /// Returns whether the slice is empty.
    /// This will always return `false` since this type by its nature cannot be empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        false
    }
}

impl<T> Index<usize> for NonEmptyMutSlice<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.slice.index(index)
    }
}

/// A string slice that is known to be non-empty
#[derive(Debug, Clone, Copy)]
pub struct NonEmptyStr<'a> {
    str: &'a str,
}

impl<'a> NonEmptyStr<'a> {
    /// Returns a new `NonEmptyStr` if `str` is not empty, otherwise `None`.
    #[must_use]
    pub const fn new(str: &'a str) -> Option<Self> {
        then!(!str.is_empty(), Self { str })
    }

    /// Returns a new `NonEmptyStr`.
    /// # Safety
    /// `str` must be known to not be empty or this could lead to undefined behavior later.
    #[must_use]
    pub const unsafe fn new_unchecked(str: &'a str) -> Self {
        Self { str }
    }

    /// Returns the first character in the string
    #[must_use]
    pub fn head(self) -> char {
        unsafe { self.str.chars().next().unwrap_unchecked() }
    }

    /// Returns a reference to the portion of the string after the first character.
    #[must_use]
    pub fn tail(self) -> &'a str {
        &self.str[1..]
    }

    /// Returns the length of the string.
    #[must_use]
    pub const fn len(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.str.len()) }
    }

    /// Returns whether the string is empty.
    ///
    /// This will always return `false` since this type by its nature cannot be empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        false
    }
}
