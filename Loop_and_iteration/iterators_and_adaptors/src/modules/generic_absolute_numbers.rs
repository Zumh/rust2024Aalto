
pub fn absolute_numbers() {
    let numbers = [1, -2, 3, -4, 5, -6, 7, -8, 9, -10];
    let absolute_numbers = numbers
        .into_iter()
        .map(i32::abs)
        .collect::<Vec<_>>();
    println!("absolute numbers: {:?}", absolute_numbers);
}