
fn main(){
    let mut arr = [91,23,1,67,23,94,12,56,78,22,6,12,45];
    merge_sort(&mut arr);
    println!("{:?}", arr);
}

fn merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    let n = arr.len();
    let m = n / 2;

    if n <= 1{
        return;
    }

    merge_sort(&mut arr[0..m]);
    merge_sort(&mut arr[m..n]);

    let mut y: Vec<T> = arr.to_vec();

    merge(&arr[0..m], &arr[m..n], &mut y[..]);
    arr.copy_from_slice(&y);

}

fn merge<T: Copy + PartialOrd>(left: &[T], right: &[T],y: &mut [T]){
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len(){
        if left[i] < right[j] {
            y[k] = left[i];
            i += 1;
            k += 1;
        } else {
            y[k] = right[j];
            j += 1;
            k += 1;
        }
    }
    if i < left.len() {
        y[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        y[k..].copy_from_slice(&right[j..]);
    }
}

