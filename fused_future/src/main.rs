use futures::future::{self, Fuse, FusedFuture, FutureExt, Ready};
use std::{pin::pin, task::{Context, Poll}};

fn main(){
    let mut future = async{42};
    let mut f = async||{};
    let f = f();

    let mut cx = Context::from_waker(futures::task::noop_waker_ref());

    let mut pined_future = pin!(future);
    assert_eq!(pined_future.as_mut().poll(&mut cx), Poll::Ready(42));
    pined_future.as_mut().poll(&mut cx);
    // assert!(pined_future.is_terminated()); // Future 已完成
    assert_eq!(pined_future.poll(&mut cx), Poll::Pending); // 不会再被轮询
}