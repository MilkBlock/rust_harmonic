use std::{borrow::Borrow, ops::Deref, sync::LazyLock};

struct S {
    i: i32,
}
trait M {}
impl M for S {}
static A: LazyLock<S> = LazyLock::new(|| S { i: 3 });
fn main() {
    println!("{}", A.i);
    f(A);
}
fn f(m: &dyn M) {}
pub struct LazyRef<T> {
    t: LazyLock<T>,
}
impl<T> LazyRef<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            t: LazyLock::new(f),
        }
    }
}
impl<T> Deref for LazyRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
impl<T> Borrow<T> for LazyRef<T> {
    fn borrow(&self) -> &T {
        &self.t
    }
}
unsafe impl<T> Sync for LazyRef<T> {}
unsafe impl<T> Send for LazyRef<T> {}
