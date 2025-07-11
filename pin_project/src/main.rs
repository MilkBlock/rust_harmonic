use std::{pin::{self, Pin}, rc::Rc, task::{Context, Poll, Waker}};
use std::pin::pin;

use async_std::future;
struct A{
    s:[bool;114514]
}

async fn bar(a:A) {
    let a = a;
    let x = async { 514 }.await;
    let y = async { 114 }.await;
    println!("hello world");
}



// impl Future for A{
//     type Output = ();
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         todo!()
//     }
// }
trait MutTrait {
    fn h(&mut self);
}
impl MutTrait for &mut A{
    fn h(&mut self) {
        todo!()
    }
}

#[tokio::main]
async fn main(){
    let mut f = bar(A{s:[true;114514]});
    let pinned = pin!(f);
    // let m = Box::new(pinned);
    let mut a = A{s:[true;114514]} ;
    check(pinned); // can't be unpined 
}
// &mut 转交过去永远不会拿走 所有权    估计在 trait 实现的时候就已经这么考虑了 
fn trans(a:&mut A) -> impl MutTrait{
    a
}
fn check(f :impl Future){
    
}
fn check_mut_trait(f :impl MutTrait){

}