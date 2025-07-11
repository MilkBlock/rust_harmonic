#![feature(coroutines, coroutine_trait)]

use core::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {

    let mut add_one = co();
    match Pin::new(&mut add_one).resume(0) {
        CoroutineState::Yielded(x) => println!("yielded {x}"),
        _ => (),
    }
    match Pin::new(&mut add_one).resume(5) {
        CoroutineState::Yielded(x) => println!("yielded {x}"),
        _ => (),
    }
}
fn co() -> impl Coroutine<i32,Yield = i32,Return = i32>{
    #[coroutine] 
    |mut x:i32| {
        loop {
            x = yield x + 1;
            if x == 3{
                break x;
            }
        }
    }
}