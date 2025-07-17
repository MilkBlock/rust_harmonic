use std::marker::PhantomData;

use bevy_reflect::{GetTypeRegistration, Reflect};

#[derive(Reflect)]
struct M<T0, T1> {
    #[reflect(ignore)] // add this to get_type_registration or you will not find that
    _p: PhantomData<(T0, T1)>,
}
#[derive(Reflect)]
struct S;
fn main() {
    let reg = M::<i32, f64>::get_type_registration();
    println!("type_info: {:?}", reg.type_info());
    println!("generics: {:?}", reg.type_info().generics());
    println!("generic: {:?}", reg.type_info().generics()[1]);
    println!("generic_type: {:?}", reg.type_info().generics()[1].ty());
}
fn print_second_generic<T: GetTypeRegistration>(t: T) {
    let reg = T::get_type_registration();
    println!("generic_type: {:?}", reg.type_info().generics()[1].ty());
}
fn l(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
