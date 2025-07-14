use frunk::{hlist::{LiftFrom, LiftInto}, HList};

type H = HList!(i32,i32,i32);
fn main(){
    let x:H = <H as LiftFrom<i32, 0> >::lift_from(1);

}