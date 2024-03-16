fn main(){
    let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    iterative_merge_sort(&mut arr);

    println!("{:?}", arr);
}

fn iterative_merge_sort(arr: &mut [i32]){
    let mut aux = vec![0; arr.len()];
    let mut size = 1;

    while size < arr.len(){
        let mut i = 0;
        let mut j = size;

        while j < arr.len(){
            merge(&mut arr[i..j], &mut aux, size);
            i = j;
            j += size;
        }
        if i < arr.len(){
            merge(&mut arr[i..], &mut aux, size);
        }
        size *= 2;
    }
}


fn merge(arr: &mut [i32], aux: &mut [i32], size: usize){
    let mut i = 0;
    let mut j = size;
    let mut k = 0;

    while i < arr.len() && j < arr.len(){
        if arr[i] < arr[j]{
            aux[k] = arr[i];
            i += 1;
        }else{
            aux[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < arr.len(){
        aux[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < arr.len(){
        aux[k] = arr[j];
        j += 1;
        k += 1;
    }

    for i in 0..arr.len(){
        arr[i] = aux[i];
    }
}
