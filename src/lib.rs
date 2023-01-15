//! [![license: MIT](https://img.shields.io/crates/l/resize_slice2)](https://github.com/benthillerkus/resize_slice/blob/main/LICENSE)
//! [![build status docs](https://img.shields.io/docsrs/resize_slice2)](https://docs.rs/resize_slice2)
//! [![crate version](https://img.shields.io/crates/v/resize_slice2)](https://crates.io/crates/resize_slice2)
//!
//! # resize slice (2)
//!
//! Enlarge and shrink slices (given a larger slice) in safe Rust.
//!
//! This is done by expressing the new slice as a slice of the source slice -- this way you can also extend the lifetime to the sources lifetime.
//!
//! # Example
//!
//! ```rust
//! use resize_slice2::ResizeSlice;
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
//! result:              |-----------|
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
pub trait ResizeSlice<'a, 'source: 'a, T, R, E> {
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
    fn resize(&'a self, source: &'source [T], by: R) -> &'source [T];

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
    fn try_resize(&'a self, source: &'source [T], by: R) -> Result<&'source [T], E>;
}

impl<'a, 'source: 'a, T> ResizeSlice<'a, 'source, T, RangeFull, Error> for &'a [T] {
    #[inline(always)]
    fn resize(&'a self, source: &'source [T], _by: RangeFull) -> &'source [T] {
        source
    }

    fn try_resize(&'a self, source: &'source [T], _by: RangeFull) -> Result<&'source [T], Error> {
        if self.is_slice_of(source) {
            Ok(source)
        } else {
            Err(Error::NotInSource)
        }
    }
}

impl<'a, 'source: 'a, T, I> ResizeSlice<'a, 'source, T, RangeFrom<I>, Error> for &'a [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&'a self, source: &'source [T], by: RangeFrom<I>) -> &'source [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let new_start = self_start.as_() + by.start;

        &source[new_start.as_()..]
    }

    fn try_resize(&'a self, source: &'source [T], by: RangeFrom<I>) -> Result<&'source [T], Error> {
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

impl<'a, 'source: 'a, T, I> ResizeSlice<'a, 'source, T, RangeTo<I>, Error> for &'source [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&'a self, source: &'source [T], by: RangeTo<I>) -> &'source [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_end = self_end.as_() + by.end;

        &source[..new_end.as_()]
    }

    fn try_resize(&'a self, source: &'source [T], by: RangeTo<I>) -> Result<&'source [T], Error> {
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

impl<'a, 'source: 'a, T, I> ResizeSlice<'a, 'source, T, Range<I>, Error> for &'a [T]
where
    I: AsPrimitive<usize> + Copy + Add<Output = I> + PartialOrd,
    usize: AsPrimitive<I>,
{
    fn resize(&'a self, source: &'source [T], by: Range<I>) -> &'source [T] {
        let self_start =
            (self.as_ptr() as usize - source.as_ptr() as usize) / std::mem::size_of::<T>();
        let self_end = self_start + self.len();
        let new_start = self_start.as_() + by.start;
        let new_end = self_end.as_() + by.end;

        &source[new_start.as_()..new_end.as_()]
    }

    fn try_resize(&'a self, source: &'source [T], by: Range<I>) -> Result<&'source [T], Error> {
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
