use std::fmt::Debug;
trait DoSomething<T> {
    fn do_sth(&self, value: T);
}
impl<'a, T:Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?} {:?}", self, value);
    }
}
fn foo(b: impl for<'b>  DoSomething<&'b usize>) {
    let s: usize = 10;
    b.do_sth(&s); // s does not live long enough 
}

trait RefTuple<'a> {
    type Output;
}
// this can be generated  
impl<'a, A:'a, B:'a> RefTuple<'a> for (A, B) {
    type Output = (&'a A, &'a B);
}

fn main() {
    // 这个问题是 x 被创建的时候 dyn 绑定了 &2 的生命周期
    let x = &2usize;
    foo(x);
}

trait TR {
    type Output: for<'a> Fn(&'a str) -> &'a str;
}
struct UP;

impl TR for UP {
    type Output = for<'a> fn(&'a str) -> &'a str;
}