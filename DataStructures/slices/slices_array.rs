// get method from array is actually from slice
// slices of array 
fn main() {
    let a = [1, 2, 3, 4, 5];
    // slices can not be modified while slice is still alive
    // only known at runtime, because they are dynamically sized
    let slice: &[i32] = &a[1..3];
    println!("{:?}", slice);

    // slices can be modified when mutable
    let mut arr = [1, 2, 3, 4, 5];
    // we borrow arr as mutable
    let arr_slice: &mut [i32] = &mut arr[1..3];
    
    // modify arr here and this is dangerous
    //arr[0] = 10;
    // instead we can modify arr_slice
    arr_slice[0] = 10;
    // we use slice here
    println!("{:?}", arr_slice);
}
