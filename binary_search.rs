pub fn binary_search(a: &[i32], len: usize, target_value: &i32) -> Option<usize>{
    let mut low : i8 = 0;
    let mut high : i8 = len as i8 - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_index = mid as usize;
        let val = &a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        if val < target_value {
            low = mid + 1;
        }

        if val > target_value {
            high = mid - 1;
        }
    }
    None
}

fn  find_smallest(arr: &[i32]) -> Option<usize>{
    let mut smallest_index = 0;

    for (j, &value) in arr.iter().enumerate() {
        if value <  arr[smallest_index]{
            smallest_index = j;
        }
    }
    Some(smallest_index)
}

fn  find_highest(arr: &[i32]) -> Option<usize>{
    let mut smallest_index = 0;

    for (j, &value) in arr.iter().enumerate() {
        if value >  arr[smallest_index]{
            smallest_index = j;
        }
    }
    Some(smallest_index)
}

fn selection_sort(arr: &mut [i32]){
    let len = arr.len();
    for i in 0..len{
        let min = (i..len).min_by_key(|x| arr[*x]).unwrap();
        arr.swap(min, i);
    }
}

fn main() {
    let mut arr = [
		14, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
    let target: i32 = 59;

    selection_sort(&mut arr);

    if let Some(result) = find_smallest(&arr){
        println!("Smallest value is {}", arr[result]);
    } else {
        println!("Something went wrong");
    }

    if let Some(result) = find_highest(&arr){
        println!("Highest value is {}", arr[result]);
    } else {
        println!("Something went wrong");
    }

    if let Some(result) = binary_search(&arr, arr.len(), &target){
        println!("{} found at index {}", target, result);
    } else {
        println!("{} not found", target);
    }
}
