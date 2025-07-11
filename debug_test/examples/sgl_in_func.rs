trait SglGetter{
    fn sgl() -> &'static Self;
}

fn f(){
    struct M{
        s:String
    }
    impl SglGetter for M {
        fn sgl() -> &'static M{
            static INSTANCE:std::sync::OnceLock<M>  = std::sync::OnceLock::new();
            let instance = &INSTANCE.get_or_init(|| {
                println!("Creating singleton instance");
                M { s: "Hello, Singleton!".to_string() }
            });
            // println!("Singleton instance created: {:?}", instance.s);
            instance
        }
    }
    M::sgl();
    println!("Singleton {} accessed successfully.", M::sgl().s);
}

fn main(){
    f();
}



