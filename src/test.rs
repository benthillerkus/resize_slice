use super::*;

#[test]
fn test_extend_slice() {
    let source = [1, 2, 3, 4, 5];
    let slice = &source[1..3];
    assert_eq!(slice, &[2, 3]);
    let extended = slice.resize(&source, 0..2);
    assert_eq!(extended, &[2, 3, 4, 5]);
}

#[test]
fn test_resize_slice() {
    let source = [1, 2, 3, 4, 5];
    let slice = &source[1..3];
    let extended = slice.resize(&source, -1..2);
    assert_eq!(extended, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_resize_slice_unbounded() {
    let source = [1, 2, 3, 4, 5];
    let slice = &source[1..3];
    let extended = slice.resize(&source, ..2);
    assert_eq!(extended, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_resize_slice_unbounded2() {
    let source = [1, 2, 3, 4, 5];
    let slice = &source[1..3];
    let extended = slice.resize(&source, ..);
    assert_eq!(extended, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_not_source_slice_lower() {
    let source = [1, 2, 3, 4, 5];
    let source2 = [6, 7, 8, 9, 10];
    let slice = &source[1..3];
    let extended = slice.try_resize(&source2, ..);
    assert_eq!(extended, Err(Error::NotInSource));
}

#[test]
fn test_not_source_slice_upper() {
    let source = [1, 2, 3, 4, 5];
    let source2 = [6, 7, 8, 9, 10];
    let slice = &source2[1..3];
    let extended = slice.try_resize(&source, ..);
    assert_eq!(extended, Err(Error::NotInSource));
}
