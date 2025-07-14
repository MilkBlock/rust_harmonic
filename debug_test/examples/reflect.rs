use std::any::{Any, TypeId};

use bevy_reflect::{Reflect, Struct, TypeInfo, TypeRegistry};

// 定义一个可反射的结构体
#[derive(Reflect)]
struct MyStruct {
    field_a: f32,
    field_b: String,
    field_c: bool,
}

fn print_field_names() {
    // 从类型注册表中获取类型信息
    let mut ty_reg = TypeRegistry::empty();
    ty_reg.register::<MyStruct>();
    let ty = ty_reg.get(TypeId::of::<MyStruct>()).unwrap();
    let type_info = ty.type_info();
    let struct_info = type_info.as_struct().unwrap();
    let field_names = struct_info.field_names();
    println!("Field names: {:?}", field_names);
}

fn main() {
    print_field_names();
}
