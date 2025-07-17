use std::marker::PhantomData;

struct A;
trait M {
    fn create_default<T>() {}
}
// generic
impl M for A {
    fn create_default<T>() {}
}
impl A {
    fn r(&self) -> &impl M {
        self
    }
}

trait M2 {
    const A: i32;
}
impl M2 for A {
    const A: i32 = 3;
}

struct F<T: Copy = i32> {
    p: PhantomData<T>,
}
impl<T: std::marker::Copy> F<T> {
    fn build<T>() {}
}

trait M3 {}
impl M3 for A {}

fn main() {
    let a = A;
    let m = a.r();
    let m: &dyn M3 = &a;
    // which is not dyn compatible
}
