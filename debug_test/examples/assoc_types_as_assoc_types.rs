use std::marker::PhantomData;

trait M1{
    type A;
}
trait M2<'a>{
    type B;
}
struct S;
impl M1 for S where S:for<'a> M2<'a>{
    type A = <S as M2<'a>>::B;
}


impl M2 for S{
    type B = i32;
}
fn main(){

}