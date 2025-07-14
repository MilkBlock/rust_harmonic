use std::mem::transmute;

trait MyTrait {
    fn speak(&self);
}

struct Cat;
impl MyTrait for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

struct Dog;
impl MyTrait for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn main() {
    let dog = Dog;
    let t: &dyn MyTrait = &dog;

    unsafe {
        // 把 &dyn MyTrait 转换为 *const Dog（危险）
        let fake_dog: &Cat = transmute(t); // 🚨 UB
        fake_dog.speak(); // 很可能崩溃或输出垃圾
    }
}
fn m(s: SS) {
    use M::SS;
}

pub mod M {
    pub struct SS {}
}
