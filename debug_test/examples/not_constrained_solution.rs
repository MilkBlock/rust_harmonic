struct M;
trait Context{}
trait MTrait<A>{ }
trait SingletonGetter<A>{ 
    fn ret() -> A;
}

trait Marker{}

impl<T1:Context,T> Marker for T:SingletonGetter<T1>{

}

impl<T1:Context,T2:SingletonGetter<T1>> MTrait<T2> for M { }
// 这里似乎只能引入 带 associative type 的  trait 来解决这个问题

fn main(){


}



