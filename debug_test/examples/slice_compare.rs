fn main() {
    let m = [1, 2, 3];
    let a = [1, 2, 3];
    let b: Result<&[i32; 1], _> = a.as_slice().try_into();
    // 注意你必须要把 a 先转化为 a.as_slice 的切片类型，而不是一个定长数组
    println!("{:?}", m[0..=1] == a);
}
