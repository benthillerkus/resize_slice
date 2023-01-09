//! # Resize Slice
//!
//! Shrink or enlarge a slice (given a larger slice) in safe Rust.
//!
//! ```rust
//! use resize_slice2::ResizeSlice;
//!
//! let source = &["a", "b", "c", "d", "e", "f"];
//! let slice = &source[1..4];
//! assert_eq!(slice, &["b", "c", "d"]);
//! let resized = slice.try_resize(source, 0..1).unwrap();
//! assert_eq!(resized, &["b", "c", "d", "e"]);
//! ```
//!
//! So a range of `1..-1` would move the start one to the right and move the end one to the left.
//! ```blank
//! source: |------------------------|
//! slice:             |-------|
//! result:              |---|
//! ```
//! A range of `1..` would move the start one to the right and fully expand the end.
//! ```blank
//! source: |------------------------|
//! slice:             |-------|
//! result:            |-------------|
//! ```
//! A range of `0..0` would return the same slice.
//! ```blank
//! source: |------------------------|
//! slice:            |-------|
//! result:           |-------|
//! ```

use num_traits::AsPrimitive;
use std::ops::{Add, Range, RangeFrom, RangeFull, RangeTo};

mod is_slice;
pub use is_slice::CouldBeSliceOf;

#[cfg(test)]
mod test;

/// An error that can occur when resizing a slice.
///
/// # Examples
///
/// ## Negative Slice
/// ```
/// # use resize_slice2::*;
/// let source = &[1, 2, 3, 4, 5];
/// let slice = &source[1..2];
/// let err = slice.try_resize(source, 1..-1).unwrap_err();
/// assert_eq!(err, Error::NegativeSlice);
/// ```
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The desired new slice would be out of bounds of the source slice.
    OutOfBounds,
    /// The start of the desired new slice would be after its end.
    /// This can only happen when working with a `Range`,
    /// as `RangeFrom` and `RangeTo` are unbounded on one side.
    NegativeSlice,
    /// The slice is not part of the source slice.
    NotInSource,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "the slice would be out of bounds of the source slice"),
            Error::NegativeSlice => write!(f, "the start of the new slice would be after its end"),
            Error::NotInSource => write!(f, "the slice is not part of the source slice"),
        }
    }
}

impl std::error::Error for Error {}

/// Allows a slice of `T` to be resized.
pub trait ResizeSlice<'a, T, R, E> {
    /// Resizes the slice using the given range `by`. May panic if the new slice is out of bounds.
    ///
    /// The start of the range is relative to the start of the slice.
    /// The end of the range is relative to the end of the slice.
    ///
    /// So a range of `1..-1` would move the start one to the right and move the end one to the left.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:             |---|
    /// ```
    /// A range of `1..` would move the start one to the right and fully expand the end.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:             |------------|
    /// ```
    /// A range of `0..0` would return the same slice.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:           |-------|
    /// ```
    fn resize(&self, source: &'a [T], by: R) -> &'a [T];

    /// Resizes the slice using the given range `by`.
    ///
    /// The start of the range is relative to the start of the slice.
    /// The end of the range is relative to the end of the slice.
    ///
    /// So a range of `1..-1` would move the start one to the right and move the end one to the left.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:             |---|
    /// ```
    /// A range of `1..` would move the start one to the right and fully expand the end.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:             |------------|
    /// ```
    /// A range of `0..0` would return the same slice.
    /// ```blank
    /// source: |------------------------|
    /// self:             |-------|
    /// result:           |-------|
    /// ```
    fn try_resize(&self, source: &'a [T], by: R) -> Result<&'a [T], E>;
}

impl<'a, T> ResizeSlice<'a, T, RangeFull, Error> for &'a [T] {
    fn resize(&self, source: &'a [T], _by: RangeFull) -> &'a [T] {
        source
    }

    fn try_resize(&self, source: &'a [T], _by: RangeFull) -> Result<&'a [T], Error> {
        if self.is_slice_of(source) {
            Ok(source)
        } else {
            Err(Error::NotInSource)
        }
    }
}

impl<'a, T, I> ResizeSlice<'a, T, RangeFrom<I>, Error> for &'a [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&self, source: &'a [T], by: RangeFrom<I>) -> &'a [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let new_start = self_start.as_() + by.start;

        &source[new_start.as_()..]
    }

    fn try_resize(&self, source: &'a [T], by: RangeFrom<I>) -> Result<&'a [T], Error> {
        if !self.is_slice_of(source) {
            return Err(Error::NotInSource);
        }
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let new_start = self_start.as_() + by.start;

        if new_start < 0usize.as_() || new_start > source.len().as_() {
            return Err(Error::OutOfBounds);
        }

        Ok(&source[new_start.as_()..])
    }
}

impl<'a, T, I> ResizeSlice<'a, T, RangeTo<I>, Error> for &'a [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&self, source: &'a [T], by: RangeTo<I>) -> &'a [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_end = self_end.as_() + by.end;

        &source[..new_end.as_()]
    }

    fn try_resize(&self, source: &'a [T], by: RangeTo<I>) -> Result<&'a [T], Error> {
        if !self.is_slice_of(source) {
            return Err(Error::NotInSource);
        }
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_end = self_end.as_() + by.end;

        if new_end < 0usize.as_() || new_end > source.len().as_() {
            return Err(Error::OutOfBounds);
        }

        Ok(&source[..new_end.as_()])
    }
}

impl<'a, T, I> ResizeSlice<'a, T, Range<I>, Error> for &'a [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&self, source: &'a [T], by: Range<I>) -> &'a [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_start = self_start.as_() + by.start;
        let new_end = self_end.as_() + by.end;

        &source[new_start.as_()..new_end.as_()]
    }

    fn try_resize(&self, source: &'a [T], by: Range<I>) -> Result<&'a [T], Error> {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_start = self_start.as_() + by.start;
        let new_end = self_end.as_() + by.end;

        if new_end < new_start {
            return Err(Error::NegativeSlice);
        } else if new_start < 0usize.as_() || new_end > source.len().as_() {
            return Err(Error::OutOfBounds);
        }

        Ok(&source[new_start.as_()..new_end.as_()])
    }
}
