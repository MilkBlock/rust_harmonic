use std::{marker::PhantomData, mem::MaybeUninit};

struct S;
impl Drop for S{
    fn drop(&mut self) {
        println!("hello drop")
    }
}
fn main(){
    let s = S;
    MaybeUninit
}


