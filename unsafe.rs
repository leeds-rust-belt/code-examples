fn main() {
    let num = 23;
    let borrowed_num = &num;
    let raw_ptr = borrowed_num as *const i32;
    unsafe {
        assert!(*raw_ptr == 23);
    }
}
