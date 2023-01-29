mod mods;
use std::ops;

fn get_last<T: ops::Mul + Copy, const N: usize>(arr: &[T; N]) -> T {
    let len = arr.len();
    arr[len-1]
}

fn main() {
    let arr: [i32; 2] = [3; 2];
    get_last(&arr);
}