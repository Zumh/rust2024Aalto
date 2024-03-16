fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let index = binary_search(&arr, 5);

    println!("{}", index);
}

fn binary_search(arr: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut mid = 0;

    while left <= right {
        mid = (left + right) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Less => {
                left = mid + 1;
            },
            std::cmp::Ordering::Greater => {
                right = mid - 1;
            },
            std::cmp::Ordering::Equal => break,
        }
    }
    mid
}


