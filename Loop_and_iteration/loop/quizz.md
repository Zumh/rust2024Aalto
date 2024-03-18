## what is the output ?

```rust
fn main() {
    let mut x = 0;
    loop {
        print!("{x} ");
        x += 2;
        if x > 10 {
            break;
        }
        if x % 3 == 0 {
            continue;
        }
    }
}
```
- Output:
```Rust
 0 2 4 6 8 10
```

## second question
```rust
fn main() {
    let mut x = 0;
    loop {
        print!("{x} ");
        x += 2;
        if x > 10 {
            break;
        }
        if x % 3 == 0 {
            continue;
        }
    }
}
```
- Output:
```Rust
nothing output
```
