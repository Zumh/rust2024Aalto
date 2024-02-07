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
- Fixed size and must be same type
- we can initialize with same value x size.
```rust
fn main() {
    let a = [1, 2, 3, 4];
    println!("{:?}", a);

    // We can also initialize an array of desired size
    // with the same value for each element
    let ones = [1; 10];
    println!("{:?}", ones);
}
```
- Array allow runtime index access with dynamic value.
- Tuple need compile time constants number for index accessing. no dyanamic runtime access.
- Array index must be unsigned integer number only.
- [T; N], where T is the type of each element and N is the number of elements in the array
- Do not initialize array in decrease or increase in size
```rust
#![allow(unused)]
fn main() {
    let mut array: [i32; 4] = [1, 2, 3, 4];
    // array = [3, 2, 1]; // type mismatch
    array = [4, 3, 2, 1];
}
```
- Error Handling
```rust
fn main() {
    let a = [1, 2, 3, 4];
    let x: Option<&i32> = a.get(3);
    println!("{x:?}"); // Some(4)
    let whoops = a.get(4);
    println!("{whoops:?}"); // None
}
```
- .get() return None if out of bound and doesn't cause panick.
- if pass then it return Some wrap the value.
- no specific mutable and imutable element.
- Moving array element string type to another variable is not allow
```rust
fn main() {
    let cardinals = [
        "blue grosbeak".to_string(),
        "lemon-spectacled tanager".to_string(),
        "northern cardinal".to_string()
    ];
    let second_bird = cardinals[1]; // error: cannot move out of a non-copy array
    println!("{}", second_bird);
```
- we can borrow with & and we can clone
- numbers are are different we can copy them instead of moving their ownership

## Slices
## Vectors
## Maps
