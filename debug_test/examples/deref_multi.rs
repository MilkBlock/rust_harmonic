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
}
