trait A: B {}
trait B: C {
    type M;
}
trait C {}
impl<T: B> A for T where <S as B>::M: C {}

struct S;
impl C for S {}
impl B for S {
    type M = i32;
}
impl C for i32 {}

fn main() {
    k(S);
}
fn k<T: A>(t: T) {}
