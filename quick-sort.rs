
fn main(){
    let mut arr: [i32; 10] = [8,23,34,10,11,21,31,14,35,22];
    let mut vec = vec![91,23,67,23,94,12,56,78,22,6,12,45];
    let last_index = arr.len() - 1;
    quick_sort(&mut arr, 0 , last_index);
    quicksort(&mut vec, 0, 11);
    print_array(&arr);
    println!("{:?}", vec);
}

fn swap(param_array: &mut [i32], i: usize, j :usize){
    let temp = param_array[i];
    param_array[i] = param_array[j];
    param_array[j] = temp;
}

fn partition(param_array: &mut [i32], start: usize, end: usize) -> i32{
    let pivot = param_array[end];
    let mut index = start;

    let mut i = start;
    while i < end {
        if param_array[i] < pivot {
            swap(param_array, i, index);
            index += 1;
        }

        i+=1;
    }
    swap(param_array, index, end);
    return index as i32;
}

fn quick_sort(param_array: &mut [i32], start: usize, end: usize){
    if start >= end {
        return;
    }
    let pivot = partition(param_array, start, end);
    println!("=={}", pivot);

    quick_sort(param_array, start, (pivot -1) as usize);
    quick_sort(param_array, (pivot + 1) as usize, end);
}

fn quicksort(mut vec: &mut Vec<usize>, low: usize, high: usize){
    if low < high {
        let pivot = partition_vec(&mut vec, low, high);
        quicksort(&mut vec, low, pivot.saturating_sub(1));
        quicksort(&mut vec, pivot + 1, high);
    }
}

fn partition_vec(vec: &mut Vec<usize>, low: usize, high: usize) -> usize {
    let pivot = vec[high];
    let mut i = low;

    for j in low..high{
        if vec[j] < pivot {
            vec.swap(i, j);
            i += 1;
        }
    }

    vec.swap(i, high);
    i
}


fn print_array(arr:&[i32]){
    println!("{:?}", arr);
}
