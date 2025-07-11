struct M;
trait Context{}
trait MTrait<A>{ }
trait SingletonGetter<A>{ 
    fn ret() -> A;
}

impl<T1:Context,T2:SingletonGetter<T1>> MTrait<T2> for M { }

fn main(){


}


