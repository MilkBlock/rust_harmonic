struct A;
const _: () = {
    pub type B = A;
    pub struct M{
        a:i32
    }
    pub const EXTERNAL_CONST: i32 = 42; // 标记为 pub
    pub fn external_fn() -> &'static str { "Hello" }
    impl A{
        fn p() -> M{
            println!("fh");
            M { a:3 }
        }
    }
};

struct S{
    a:i32,
    b:i32,
}
// fn main() {
//     let mut s = A::p();
//     s.a =2;
//     let a = 5;
//     // println!("{}", EXTERNAL_CONST); // 不能访问
//     // println!("{}", external_fn());  // 不能访问
// }
mod message {
    pub enum Message {
        Quit,
        Move { x: i32, y: i32 }, // 注意：字段不是 `pub`！
        Write(String),
    }

    impl Message {
        // 提供构造函数
        pub fn move_to(x: i32, y: i32) -> Self {
            Message::Move { x, y }
        }
    }
}

fn main() {
    // ✅ 允许：通过构造函数创建
    let msg = message::Message::move_to(1, 2);

    // ❌ 编译错误：字段 `x` 和 `y` 是私有的
    let msg = message::Message::Move { x: 1, y: 2 };
}