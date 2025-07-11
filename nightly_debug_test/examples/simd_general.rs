#![feature(portable_simd)]
#![feature(core_float_math)]
use core::f32::math::sqrt;
use std::{hint::black_box, simd::{f32x16, f32x4, i32x4, StdFloat}, time::Instant};

static SIZE:usize=128000;
fn main(){
    let mut arr = black_box([1.;SIZE]);

    let start = Instant::now();
    // 标量版本
    for x in &mut arr { *x = sqrt((*x* 3. + 7.)); }
    println!("common {:?} ans:{:?}", start.elapsed(), arr[1000]);

    let mut arr = black_box([1.;SIZE]);
    let start = Instant::now();
    // SIMD 版本
    let three = f32x16::splat(3.);
    let seven = f32x16::splat(7.);
    for v in arr.as_simd_mut::<16>().1.iter_mut() {
        *v = (*v * three + seven).sqrt();
    }
    println!("simd {:?} ans:{:?}", start.elapsed(), arr[1000]);
}