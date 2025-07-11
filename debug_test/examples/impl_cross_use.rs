trait A:B{
    fn a(){}
}
trait B{
    fn b(){}
}

struct S;

// impl B for S {
//     fn b(){
//         Self::a();
//     }
// }
impl A for S{ }



struct S2;

impl A for S2{

}
impl<T:A> B for T{
}



trait TY {
    const TY_NAME:&'static str ;
}
impl TY for S{
    const TY_NAME:&'static str = "hello" ;
}


fn main(){
    let a = S;
}
