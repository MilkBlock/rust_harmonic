use std::{marker::PhantomData, mem::MaybeUninit};

struct S;
trait F{}
impl F for S{};
impl Drop for H{

}
impl Drop for S {
    fn drop(&mut self) {
        println!("hello drop")
    }
}
fn main() {
    let s = S;
    MaybeUninit
}
