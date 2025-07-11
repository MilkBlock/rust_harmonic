use std::marker::PhantomData;

struct S<T>{
    _p: PhantomData<T>
}

trait K{}
trait M{}
impl M for S<()>{

}
// this can't compile because the speciality of Drop
impl<T> Drop for S<T> where Self: M{
    fn drop(&mut self) {
        println!("hello drop")
    }
}
// can compile
trait C{ }
impl<T> C for S<T> where Self: M{

}
fn main(){
    let s = S::<()>{ _p: PhantomData };
    let h = H::<i32>{
        _p: PhantomData,
    };
}

// Even you can add trait bounds to Self ?
struct H<T:Drop> where Self:G{
    _p :PhantomData<T>
}

// can't compile because you should at least restrict T with H's decl restriction
trait G{
    fn B();
}
// impl<T> G for H<T>{
//     fn B() {
//         todo!()
//     }
// }

impl<T> C for H<T>{

}
impl C for H{

}
// it seems that Self restriction is unneeded in trait impl
// you may only restrict them with restrictions except Self's restriction