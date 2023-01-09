# Resize Slice

Shrink or enlarge a slice (given a larger slice) in safe Rust.

```rust
use resize_slice2::ResizeSlice;
let source = &["a", "b", "c", "d", "e", "f"];
let slice = &source[1..4];
assert_eq!(slice, &["b", "c", "d"]);
let resized = slice.try_resize(source, 0..1).unwrap();
assert_eq!(resized, &["b", "c", "d", "e"]);
```

So a range of `1..-1` would move the start one to the right and move the end one to the left.
```blank
source: |------------------------|
slice:             |-------|
result:              |---|
```
A range of `1..` would move the start one to the right and fully expand the end.
```blank
source: |------------------------|
slice:             |-------|
result:            |-------------|
```
A range of `0..0` would return the same slice.
```blank
source: |------------------------|
slice:            |-------|
result:           |-------|
```