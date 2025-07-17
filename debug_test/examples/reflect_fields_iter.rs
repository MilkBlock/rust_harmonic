use std::{any::{Any, TypeId}, marker::PhantomData};
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Struct, TypeInfo, TypeRegistry};

#[derive(Reflect)]
struct M<T>{
    t:T,
    #[reflect(ignore)]
    _p:PhantomData<i32>
}
type MA = M<i32>;

// 定义一个可反射的结构体
#[derive(Reflect)]
struct MyStruct<T> {
    field_a: f32,
    field_b: String,
    field_c: bool,
    field_d: M<T>,
    // ignore all aliases
    field_e: MA,
}

fn print_field_names() {
    // 从类型注册表中获取类型信息
    let type_reg = MyStruct::<i64>::get_type_registration();
    let type_info = type_reg.type_info();
    let struct_info = type_info.as_struct().unwrap();
    let field_names = struct_info.field_names();

    struct_info.iter().for_each(|x|println!("{:?},{:?}", x.name(), x.ty().ident()));
}

fn main() {
    print_field_names();
}

