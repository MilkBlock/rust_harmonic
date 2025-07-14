use std::marker::PhantomData;

use frunk::labelled::field_with_name;
use frunk::{labelled::Field, path::PathTraverser, LabelledGeneric};
use frunk::hlist::{HCons, HNil};

// 定义一个结构体
#[derive(LabelledGeneric)]
struct Person {
    name: String,
    age: u32,
}

fn get_field_names() -> Vec<&'static str> {

    type A = <Person as LabelledGeneric>::Repr;
    field_with_name(name, value)
    fn traverse<H, T>( p:PhantomData<HCons<H,T>>,names: &mut Vec<&'static str>) {
        HCons::<H,T>::
    }

    // let labelled = T::labelled_generic();
    // let a = T::Repr::default();
    // let mut names = Vec::new();
    
    // // 递归遍历 HList 获取字段名
    // fn traverse<H, T>(hlist: HCons<H, T>, names: &mut Vec<&'static str>) 
    // where
    //     H: Field,
    //     T: frunk::HList,
    // {
    //     names.push(H::key());
    //     match hlist.tail {
    //         frunk::HNil => (),
    //         t => traverse(t, names),
    //     }
    // }
    
    // traverse(labelled, &mut names);
    // names
}

fn main() {
    let field_names = get_field_names::<Person>();
    println!("Field names: {:?}", field_names); // 输出: ["name", "age"]
}