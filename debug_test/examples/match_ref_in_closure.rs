use std::{marker::PhantomData, ops::ControlFlow};

use frunk::Coprod;

struct S {
    i: i32,
}
trait M {}
fn S1<T1: M, F: Fn(T1)>() {
    // p:PhantomData<T1>,
    // p1:PhantomData<F>,
}
fn main() {
    let mut m = S { i: 3 };
    let mut s = 3;
    let mut a = || {
        &mut s;
        &m
    };
    let k = a();
    let k = a();
    let k = a();
    drop(m);
    let m = a();
}
