#![feature(portable_simd)]
use std::{hint::black_box, ops::{AddAssign, Mul, MulAssign}, simd::{i32x16, i32x32, i32x4, Simd}, time::{Duration, Instant}};
#[inline(always)]  
fn my_function(arr: &mut [i32]) {
    arr.iter_mut().for_each(
        |x| {
            *x=(*x)*(*x) + 3*(*x) + 2;
        }
    );
}
// x = x^2 + 3x + 2
#[inline(always)]  
fn my_function_simd(arr: &mut [i32],    ) {
    let (pre,mid,suf) = arr.as_simd_mut::<16>();
    mid.iter_mut().for_each(|x| {
            *x = (*x)*(*x) + (*x)*i32x16::splat(3) + i32x16::splat(2);
        }
    );
}
// mention that above function can speed up because 
// 1. the memory operation is not blocked
// 2. 
fn main(){
    let start = Instant::now();
    let mut arr = [1;128000];
    black_box(my_function(&mut arr));
    println!("common {:?} ans:{:?}", start.elapsed(), arr[1000]);

    let start = Instant::now();
    let mut arr = [1;128000];
    black_box(my_function_simd(&mut arr));
    println!("simd {:?} ans:{:?}", start.elapsed(), arr[1000]);
}

//cargo rustc --release -- --emit asm