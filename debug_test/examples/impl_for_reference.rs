use std::{borrow::Borrow, ops::Deref};

struct S;
trait M{
    fn f(&mut self){}
}
impl M for S{
    fn f(&mut self){
        self.deref();
    }
}
pub trait T{
    type H<'a,T:M + 'a > : Deref<Target =T>;
}
impl T for S{
    type H<'a,S1:M + 'a> =&'a S1;
}

trait Node{}
trait F{
    type Input;
    type Output;
    type OutputRef<'a> : Deref<Target = Self::Output>;
}

struct ANode{}
impl Node for ANode{}

struct F1;
impl F for F1{
    type Input = i32;
    type Output = ANode;
    type OutputRef<'a> = &'a ANode;
}

fn main(){
    let s = S;
    ch::<S>(&s)
}
fn ch<T0:T>(h:T0::OutpuRef){

}