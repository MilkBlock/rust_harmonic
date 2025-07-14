use std::marker::PhantomData;

#[derive(Debug)]
struct A<T> {
    t: T,
    _p: PhantomData<T>,
}
fn main() {}
