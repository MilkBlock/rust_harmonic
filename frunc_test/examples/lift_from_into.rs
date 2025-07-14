use frunk::lift_from;
use frunk::prelude::*;
use frunk_core::{HList, hlist};

type H = HList![(), usize, f64, (), bool];

fn main(){
    let x = H::lift_from(42.0);
    assert_eq!(x, hlist![(), 0, 42.0, (), false]);

    let x: H = lift_from(true);
    assert_eq!(x, hlist![(), 0, 0.0, (), true]);

    let x:H = lift_from((1,2,));
}