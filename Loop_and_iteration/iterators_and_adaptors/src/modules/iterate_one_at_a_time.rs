/*
The next method yields values of type Option. 
After all, there might not be a next value in the iterator. 
We can see this when calling next() thrice for an array of size 2 in the above example.

Notice that we defined the arr_iter variable as mutable. 
This is required even though the array itself doesn't change — the iterator does change.
 The iterator "keeps getting smaller" each time we take the next value from it. 
 The next method immediately consumes a value from the iterator, meaning that is is not lazy. 
 The consuming iterator methods like collect and nth work non-lazily, i.e. eagerly, by calling next repeatedly.
 */
pub fn iterate_one_at_a_time() {
    let arr = [1, 2];
    let mut arr_iter = arr.iter();
    let one = arr_iter.next();
    let two = arr_iter.next();
    let three = arr_iter.next();
    println!("{one:?}, {two:?}, {three:?}");
}
