## Choosing DS
- Common ds like tuples, arrays, slices, vectors and maps depends on speed and memory.
- We use them for organizing data.
## Tuples
- Value may be different types
- Cannot grow or shrink
```rust
fn main() {
    let tupleception = (
        "one",
        2,
        (
            "Yo",
            "dawg",
            "I",
            "heard",
            "you",
            "like",
            "tuples,",
            ("so", "I", "put", "a", "tuple", "inside", "a", "tuple"),
        ),
    );
    println!("{tupleception:?}");
}

```
- return min of two options
```rust
/// Returns the minimum of two `Option<i32>` values.
/// When either of the values is `None`, returns the other value.
/// When both values are `None`, returns `None`.
fn min_option((a, b): (Option<i32>, Option<i32>)) -> Option<i32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn main() {
    let min = min_option((Some(5), Some(3)));
    println!("min: {:?}", min);
    let min = min_option((None, Some(3)));
    println!("min: {:?}", min);
}
```
- mutable tuples
```rust
fn move_up(point: &mut (i32, i32)) {
    point.1 += 1;
}
```
```rust
fn move_up((_, y): &mut (i32, i32)) {
    *y += 1;
}
```
- ownership
- to_owned() create a owned String struct of three.
- Which can be a problem if we don't want to move ownership.
```rust
let mut t = (1, 2.0, "three".to_owned());
let t2 = t.clone();
t.0 = 0;
```
```rust
let mut t = (1, 2.0, "three");
let t2 = t;
t.0 = 3;
```
## Arrays
## Slices
## Vectors
## Maps
