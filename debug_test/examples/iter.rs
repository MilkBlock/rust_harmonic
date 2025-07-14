use std::{collections::HashMap, str::FromStr};

// 什么是迭代器
// 对数组访问的抽象
fn main() {
    let a = [1, 2, 3];
    println!("{}", a[0]);
    // 很少会有单元素访问

    // 复制开销
    let s = String::from_str("xxx").unwrap();
    fn print_string(s: String) {
        println!("{}", s);
    }
    print_string(s.clone()); // 传值，复制开销
    print_string(s); // 传值，复制开销
    // 这种情况下你应该使用引用
    // 这样就不存在复制开销
    fn print_string_ref(s: &String) {
        println!("{}", s);
    }

    let s = String::from_str("xxx").unwrap();
    print_string_ref(&s); // 传引用，复制开销
    print_string_ref(&s);
    print_string_ref(&s);

    // 额外的函数调用也是开销

    // 零开销原则，返回的是引用而不是本体
    // 本体可能会存在较大的复制开销
    // cursor
    for i in a.iter() {
        println!("{}", i);
    }

    // 迭代器的好处 : 可以提供函数式调用的办法
    // 什么是函数式，把处理过程抽象成一道一道的流水线
    // 比如我们现在要做一个操作

    //考考你，给定 [1,2,3,4,5] 这个数组，你怎么打印
    // 其中所有奇数的两倍     比如在这里你应该打印  2 6 10
    // 那么我们在这里可以抽象成两道流水

    // 第一道是筛选出其中的奇数
    // 第二道 就是 x2
    // 第三道 就是 打印出来的结果
    let a = [1, 2, 3, 4, 5];
    a.iter()
        .filter(|&x| x % 2 == 1) // 筛选出奇数
        // .map(|x| x * 2) // 每个元素乘以2
        .map(mul_2) // 每个元素乘以2
        .for_each(|x| println!("{}", x)); // 打印结果

    let mut a = HashMap::new();
    a.insert("a", 1);
    a.insert("b", 1);
    for i in a.iter() {
        // :? 调用的是 Debug的 字符串格式化方法
        println!("{:?}", i)
        // :? 调用的是 Display的 字符串格式化方法
    }
}
// 你不能为一个标准库的类型实现标准库的trait
// 自己的结构体才能实现标准库的trait

// |x| x*2  这是一个闭包，就是一个匿名函数
fn mul_2(x: &i32) -> i32 {
    2 * x
}
