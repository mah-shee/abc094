#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x_ori : [usize; n]
    }
    let mut x = x_ori.clone();
    x.sort();
    for i in 0..n {
        if x_ori[i] >= x[n / 2] {
            println!("{} ", x[(n - 1) / 2]);
        } else {
            println!("{} ", x[n / 2]);
        }
    }
}
