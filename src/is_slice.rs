/// Used to check if a slice is a slice of another slice.
///
/// # Examples
/// ```
/// # use resize_slice2::CouldBeSliceOf;
/// let source = &[1, 2, 3, 4, 5];
/// let slice = &source[1..3];
///
/// assert!(slice.is_slice_of(source));
///
/// let b = &[6, 7, 8];
/// assert!(!source.is_slice_of(b));
/// ```
pub trait CouldBeSliceOf<T> {
    /// Returns `true` if `self` could be a slice of `source`.
    fn is_slice_of(&self, source: &[T]) -> bool;
}

impl<T> CouldBeSliceOf<T> for &[T] {
    #[inline]
    fn is_slice_of(&self, source: &[T]) -> bool {
        let outer_start = source.as_ptr() as usize;
        let outer_end = outer_start + source.len() * std::mem::size_of::<T>();
        let inner_start = self.as_ptr() as usize;
        let inner_end = inner_start + self.len() * std::mem::size_of::<T>();

        outer_start <= inner_start && inner_end <= outer_end
    }
}

impl<T, const N: usize> CouldBeSliceOf<T> for &[T; N] {
    #[inline]
    fn is_slice_of(&self, source: &[T]) -> bool {
        let outer_start = source.as_ptr() as usize;
        let outer_end = outer_start + source.len() * std::mem::size_of::<T>();
        let inner_start = self.as_ptr() as usize;
        let inner_end = inner_start + self.len() * std::mem::size_of::<T>();

        outer_start <= inner_start && inner_end <= outer_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    #[test]
    fn simple() {
        let a = &SOURCE;
        let b = &a[1..3];

        assert!(b.is_slice_of(a));
    }

    #[test]
    fn simple_not() {
        let a = &SOURCE[..5];
        let b = &SOURCE[5..];

        assert!(!b.is_slice_of(a));
    }

    #[test]
    fn same_to() {
        let a = &SOURCE;
        let b = &a[..5];

        assert!(b.is_slice_of(a));
    }

    #[test]
    fn same_from() {
        let a = &SOURCE;
        let b = &a[5..];

        assert!(b.is_slice_of(a));
    }

    #[test]
    fn same() {
        let a = SOURCE.as_ref();
        let b = SOURCE.as_ref();

        assert!(b.is_slice_of(a));
    }

    #[test]
    fn same_empty() {
        let a = &SOURCE[0..0];
        let b = &SOURCE[0..0];

        assert!(b.is_slice_of(a));
    }
}
