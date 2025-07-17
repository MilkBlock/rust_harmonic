use std::ops::Deref;

use derive_more::{Deref, DerefMut};

#[derive(Clone)]
struct C;
#[derive(DerefMut, Deref, Clone)]
struct B(C);

#[derive(DerefMut, Deref, Clone)]
#[deref(forward)]
struct A(B);

trait M {
    fn m(&self) {}
}
impl M for B {}
impl M for A {}

fn main() {
    let mut a = A(B(C));
    let b = B(C);

    let m = &*a.clone();
    // let a:&mut C = **A;
    **a = C;
    *a = b;

    let k = K(3);
    // 这说明编译器会自动在 变量的左边加上 *
    read_i32(&*k);
    read_i32(&k);
}

#[derive(Deref)]
struct K(i32);

fn read_i32(a: &i32) {}
