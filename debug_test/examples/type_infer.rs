use frunk::{from_labelled_generic, hlist::{self, HList, LiftFrom, LiftInto}, HList};

fn main(){
    f::<S>(
        |x| {
            Box::new(x[0].clone())
        }
    )

}

#[derive(Clone)]
struct S;
trait M{
    type Input;
    type Output;
}
impl M for S{
    type Input = Vec<S>;
    type Output = Box<S>;
}
fn f<T:M>(f:impl Fn(T::Input) -> T::Output) {
    type A = HList![i32,];
    A::LEN; //  获取list 的长度，这个会很有用
    let i = A::lift_from(3);
    // i.lift_into();
    let m = <A as LiftInto<i32, 0>>::lift_into(i);
    println!("{:?}", i)
   
    // type H = HList![(), usize, f64, (), bool];
   
    // let x = H::lift_from(42.0);
    // assert_eq!(x, hlist![(), 0, 42.0, (), false]);
   
    // let x: H = lift_from(true);
    // assert_eq!(x, hlist![(), 0, 0.0, (), true]);


}
