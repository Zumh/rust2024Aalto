// Write the function `exclamated_bytes` that takes a slice of bytes and returns a vector of bytes with an exclamation mark added to the end.

fn main() {
    let bytes = [b'a', b'b', b'c'];
    let exclamated_bytes = exclamated_bytes(&bytes);
    println!("{:?}", exclamated_bytes);
    println!("{:?}", String::from_utf8_lossy(&exclamated_bytes));
}

fn exclamated_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut result = bytes.to_vec();
    result.push(b'!');
    result
}
