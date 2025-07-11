trait A{ fn hello(){println!("hello")}}

struct Demo{

}

impl<T> A for T where T:Default{
    
}
fn main(){
    let d  = Demo{};
}