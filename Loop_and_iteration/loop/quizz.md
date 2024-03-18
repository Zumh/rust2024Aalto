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

## third question
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
 2 4 8 10
```

## fourth question
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
x is not greater than 10
print nothing
```

## fifth question
```rust
fn main() {
    let mut x = 0;
    while x < 10 {
        if x % 3 == 0 {
            x += 2;
            continue;
        }
        print!("{x} ");
        x += 2;
    }
}
```
- Output:
- never chance to print 10
```Rust

 2 4 8
```
