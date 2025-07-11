struct A;
struct B;
trait B2<T>{
}
impl B2<A> for B{
}

fn main(){
    let b = B;
    let a:Vec<&dyn B2<A>>= vec![&b];
    let a = [1,2,3,4];
}

fn de(a:&[i32]){
    let [a,b,c] = a;
}
