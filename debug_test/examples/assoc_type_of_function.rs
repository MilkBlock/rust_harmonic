fn apply_func(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
fn double<T:std::ops::Mul<i32,Output = T>>(x: T) -> T {
    x * 2
}
fn apply_func1(f: fn() -> i32) -> i32 {
    f()
}
fn double1<T:std::ops::Mul<i32,Output = T>>() -> i32 {
    2
}

trait M{}

impl M for fn(){
    
}

fn main() {
    // double 在这里会自动强转为函数指针
    let result = apply_func1(double1::<i32>);
    println!("Result: {}", result); // 输出: Result: 10
}