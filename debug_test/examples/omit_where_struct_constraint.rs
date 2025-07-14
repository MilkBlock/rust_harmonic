use std::marker::PhantomData;

trait M {}
trait M1 {
    fn g();
}
/// 这个 where 相当于是 编译期的 assert 语句,
/// Equivalent to Compile time assert statement, is not S's generic constraint so you don't need to add it
struct S<T>
where
    Self: M1,
    i32: M,
{
    _p: PhantomData<T>,
}
impl M for i32 {}
impl<T: M> M1 for S<T> {
    fn g() {}
}

trait M2 {
    fn f();
}
impl<T: M> M2 for S<T> {
    fn f() {
        Self::g();
    }
}

fn main() {
    let s = S::<i32> { _p: PhantomData };
}
