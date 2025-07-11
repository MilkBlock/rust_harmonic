#![feature(portable_simd)]
use std::{cmp::min_by, hint::black_box, ops::{AddAssign, Mul, MulAssign}, simd::{i32x16, i32x32, i32x4, num::SimdInt, Simd}, time::{Duration, Instant}};
use itertools::izip;

static SIZE:usize=128000;
#[inline(always)]  
fn my_function(arr0: &mut [i32], arr1:&[i32]) { 
    izip!(arr0, arr1).for_each(
        |(x,y)| {
            *x=x.pow(5) + 3*(*x) + 2;
            *x = *x* y.pow(2);
            *x = (*x).min(*y)
        }
    );
}
// x = x^2 + 3x + 2
#[inline(always)]  
fn my_function_simd(arr0: &mut [i32], arr1:&[i32]   ) {
    let (pre,mid0,suf) = arr0.as_simd_mut::<16>();
    let (pre,mid1,suf) = arr1.as_simd::<16>();
    izip!(mid0,mid1).for_each(|(x,y)| {
            *x = ((*x)*(*x)*(*x)*(*x)*(*x) + (*x)*i32x16::splat(3) + i32x16::splat(2))*y*y;
            *x = (*x).min(*y)
        }
    );
}
// mention that above function can speed up because 
// 1. the memory operation is not blocked
// 2. 
fn main(){
    let mut arr0 = black_box([1;SIZE]);
    let arr1 = black_box([2;SIZE]);

    let start = Instant::now();
    my_function(&mut arr0, &arr1);
    println!("common {:?} ans:{:?}", start.elapsed(), arr0[1000]);

    let mut arr0 = black_box([1;SIZE]);
    let arr1 = black_box([2;SIZE]);

    let start = Instant::now();
    black_box(my_function_simd(&mut arr0,&arr1 ));
    println!("simd {:?} ans:{:?}", start.elapsed(), arr0[1000]);
}

//cargo rustc --release -- --emit asm
