use std::{any::{Any, TypeId}, marker::PhantomData};

use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Struct, TypeInfo, TypePath, TypeRegistry};


#[derive(Reflect)]
// #[reflect(from_reflect=false)]
struct Foo<T> {
    v:Vec<T>
}



#[derive(Reflect)]
struct K;
impl Default for K{
    fn default() -> Self {
        Self {  }
    }
}
// 被 ignore 就需要实现 default 
// 不被 ignore 就需要实现 reflect
// 必须二选一啊
#[derive(Reflect)]
struct M<T>{
    t:T,
    k:K,
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

    println!("Field names: {:?}", field_names);
    let field = struct_info.field("field_a").unwrap();
    let field_d = struct_info.field("field_d").unwrap();
    let field_e = struct_info.field("field_e").unwrap();
    let ty = field.ty();

    println!("{:?}", ty.crate_name());
    println!("{:?}", field.type_info());
    println!("{:?}", ty.ident());
    println!("{:?}", field_d);
    println!("{:?}", field_e);
}

fn main() {
    print_field_names();
}
