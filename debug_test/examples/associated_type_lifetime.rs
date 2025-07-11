trait A<'a>{
    type T:M+'a;
}
trait M {

}
impl M for (&i32,&i32){

}
impl M for (&i32,){

}
impl M for (&i32){

}
// impl<'a> M for &'a i32{
    
// }
struct K;

impl<'a> A<'a> for K{
    type T = (&'a AsRef< i32>,&'a AsRef<i32>) ;
}
fn main(){
    let a = (3);
    let a = (3,);
    
}